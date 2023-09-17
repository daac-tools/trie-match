//! # `trie_match! {}`
//!
//! This macro speeds up Rust's `match` expression for comparing strings by using a compact
//! double-array data structure.
//!
//! ## Usage
//!
//! Simply wrap the existing match expression with the `trie_match! {}` macro as
//! follows:
//!
//! ```
//! use trie_match::trie_match;
//!
//! let x = "abd";
//!
//! trie_match! {
//!     match x {
//!         "a" => { println!("x"); }
//!         "abc" => { println!("y"); }
//!         "abd" | "bcc" => { println!("z"); }
//!         "bc" => { println!("w"); }
//!         _ => { println!(" "); }
//!     }
//! }
//! ```
//!
//! ## Limitations
//!
//! The followings are different from the normal `match` expression:
//!
//! * Only supports strings, byte strings, and u8 slices as patterns.
//! * The wildcard is evaluated last. (The normal `match` expression does not
//!   match patterns after the wildcard.)
//! * Pattern bindings are unavailable.
//! * Attributes for match arms are unavailable.
//! * Guards are unavailable.
mod trie;

extern crate proc_macro;

use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Arm, Error, Expr, ExprLit, ExprMatch, Lit, Pat, PatOr,
    PatReference, PatSlice, PatWild,
};

static ERROR_UNEXPECTED_PATTERN: &str =
    "`trie_match` only supports string literals, byte string literals, and u8 slices as patterns";
static ERROR_ATTRIBUTE_NOT_SUPPORTED: &str = "attribute not supported here";
static ERROR_GUARD_NOT_SUPPORTED: &str = "match guard not supported";
static ERROR_UNREACHABLE_PATTERN: &str = "unreachable pattern";
static ERROR_PATTERN_NOT_COVERED: &str = "non-exhaustive patterns: `_` not covered";
static ERROR_EXPECTED_U8_LITERAL: &str = "expected `u8` integer literal";

use crate::trie::Sparse;

/// Converts a literal pattern into a byte sequence.
fn convert_literal_pattern(pat: &ExprLit) -> Result<Option<Vec<u8>>, Error> {
    let ExprLit { attrs, lit } = pat;
    if let Some(attr) = attrs.first() {
        return Err(Error::new(attr.span(), ERROR_ATTRIBUTE_NOT_SUPPORTED));
    }
    match lit {
        Lit::Str(s) => Ok(Some(s.value().into())),
        Lit::ByteStr(s) => Ok(Some(s.value())),
        _ => Err(Error::new(lit.span(), ERROR_UNEXPECTED_PATTERN)),
    }
}

/// Converts a slice pattern into a byte sequence.
fn convert_slice_pattern(pat: &PatSlice) -> Result<Option<Vec<u8>>, Error> {
    let PatSlice { attrs, elems, .. } = pat;
    if let Some(attr) = attrs.first() {
        return Err(Error::new(attr.span(), ERROR_ATTRIBUTE_NOT_SUPPORTED));
    }
    let mut result = vec![];
    for elem in elems {
        match elem {
            Pat::Lit(ExprLit { attrs, lit }) => {
                if let Some(attr) = attrs.first() {
                    return Err(Error::new(attr.span(), ERROR_ATTRIBUTE_NOT_SUPPORTED));
                }
                match lit {
                    Lit::Int(i) => {
                        let int_type = i.suffix();
                        if int_type != "u8" && !int_type.is_empty() {
                            return Err(Error::new(i.span(), ERROR_EXPECTED_U8_LITERAL));
                        }
                        result.push(i.base10_parse::<u8>()?);
                    }
                    Lit::Byte(b) => {
                        result.push(b.value());
                    }
                    _ => {
                        return Err(Error::new(elem.span(), ERROR_EXPECTED_U8_LITERAL));
                    }
                }
            }
            _ => {
                return Err(Error::new(elem.span(), ERROR_EXPECTED_U8_LITERAL));
            }
        }
    }
    Ok(Some(result))
}

/// Checks a wildcard pattern and returns `None`.
///
/// The reason the type is `Result<Option<Vec<u8>>, Error>` instead of `Result<(), Error>` is for
/// consistency with other functions.
fn convert_wildcard_pattern(pat: &PatWild) -> Result<Option<Vec<u8>>, Error> {
    let PatWild { attrs, .. } = pat;
    if let Some(attr) = attrs.first() {
        return Err(Error::new(attr.span(), ERROR_ATTRIBUTE_NOT_SUPPORTED));
    }
    Ok(None)
}

/// Converts a reference pattern (e.g. `&[0, 1, ...]`) into a byte sequence.
fn convert_reference_pattern(pat: &PatReference) -> Result<Option<Vec<u8>>, Error> {
    let PatReference { attrs, pat, .. } = pat;
    if let Some(attr) = attrs.first() {
        return Err(Error::new(attr.span(), ERROR_ATTRIBUTE_NOT_SUPPORTED));
    }
    match &**pat {
        Pat::Lit(pat) => convert_literal_pattern(pat),
        Pat::Slice(pat) => convert_slice_pattern(pat),
        Pat::Reference(pat) => convert_reference_pattern(pat),
        _ => Err(Error::new(pat.span(), ERROR_UNEXPECTED_PATTERN)),
    }
}

/// Retrieves pattern strings from the given token.
///
/// None indicates a wild card pattern (`_`).
fn retrieve_match_patterns(pat: &Pat) -> Result<Vec<Option<Vec<u8>>>, Error> {
    let mut pats = vec![];
    match pat {
        Pat::Lit(pat) => pats.push(convert_literal_pattern(pat)?),
        Pat::Slice(pat) => pats.push(convert_slice_pattern(pat)?),
        Pat::Wild(pat) => pats.push(convert_wildcard_pattern(pat)?),
        Pat::Reference(pat) => pats.push(convert_reference_pattern(pat)?),
        Pat::Or(PatOr {
            attrs,
            leading_vert: None,
            cases,
        }) => {
            if let Some(attr) = attrs.first() {
                return Err(Error::new(attr.span(), ERROR_ATTRIBUTE_NOT_SUPPORTED));
            }
            for pat in cases {
                match pat {
                    Pat::Lit(pat) => pats.push(convert_literal_pattern(pat)?),
                    Pat::Slice(pat) => pats.push(convert_slice_pattern(pat)?),
                    Pat::Wild(pat) => pats.push(convert_wildcard_pattern(pat)?),
                    Pat::Reference(pat) => pats.push(convert_reference_pattern(pat)?),
                    _ => {
                        return Err(Error::new(pat.span(), ERROR_UNEXPECTED_PATTERN));
                    }
                }
            }
        }
        _ => {
            return Err(Error::new(pat.span(), ERROR_UNEXPECTED_PATTERN));
        }
    }
    Ok(pats)
}

struct MatchInfo {
    bodies: Vec<Expr>,
    pattern_map: HashMap<Vec<u8>, usize>,
    wildcard_idx: usize,
}

fn parse_match_arms(arms: Vec<Arm>) -> Result<MatchInfo, Error> {
    let mut pattern_map = HashMap::new();
    let mut wildcard_idx = None;
    let mut bodies = vec![];
    for (
        i,
        Arm {
            attrs,
            pat,
            guard,
            body,
            ..
        },
    ) in arms.into_iter().enumerate()
    {
        if let Some(attr) = attrs.first() {
            return Err(Error::new(attr.span(), ERROR_ATTRIBUTE_NOT_SUPPORTED));
        }
        if let Some((if_token, _)) = guard {
            return Err(Error::new(if_token.span(), ERROR_GUARD_NOT_SUPPORTED));
        }
        let pat_bytes_set = retrieve_match_patterns(&pat)?;
        for pat_bytes in pat_bytes_set {
            if let Some(pat_bytes) = pat_bytes {
                if pattern_map.contains_key(&pat_bytes) {
                    return Err(Error::new(pat.span(), ERROR_UNREACHABLE_PATTERN));
                }
                pattern_map.insert(pat_bytes, i);
            } else {
                if wildcard_idx.is_some() {
                    return Err(Error::new(pat.span(), ERROR_UNREACHABLE_PATTERN));
                }
                wildcard_idx.replace(i);
            }
        }
        bodies.push(*body);
    }
    let Some(wildcard_idx) = wildcard_idx else {
        return Err(Error::new(Span::call_site(), ERROR_PATTERN_NOT_COVERED));
    };
    Ok(MatchInfo {
        bodies,
        pattern_map,
        wildcard_idx,
    })
}

fn trie_match_inner(input: ExprMatch) -> Result<TokenStream, Error> {
    let ExprMatch {
        attrs, expr, arms, ..
    } = input;
    let MatchInfo {
        bodies,
        pattern_map,
        wildcard_idx,
    } = parse_match_arms(arms)?;
    let mut trie = Sparse::new();
    for (k, v) in pattern_map {
        if v == wildcard_idx {
            continue;
        }
        trie.add(k, v);
    }
    let (bases, checks, outs) = trie.build_double_array_trie(wildcard_idx);

    let base = bases.iter();
    let out_check = outs.iter().zip(checks).map(|(out, check)| {
        let out = format_ident!("V{out}");
        quote! { (__TrieMatchValue::#out, #check) }
    });
    let arm = bodies.iter().enumerate().map(|(i, body)| {
        let i = format_ident!("V{i}");
        quote! { __TrieMatchValue::#i => #body }
    });
    let attr = attrs.iter();
    let enumvalue = (0..bodies.len()).map(|i| format_ident!("V{i}"));
    let wildcard_ident = format_ident!("V{wildcard_idx}");
    Ok(quote! {
        {
            #[derive(Clone, Copy)]
            enum __TrieMatchValue {
                #( #enumvalue, )*
            }
            #( #attr )*
            match (|query: &[u8]| unsafe {
                let bases: &'static [i32] = &[ #( #base, )* ];
                let out_checks: &'static [(__TrieMatchValue, u8)] = &[ #( #out_check, )* ];
                let mut pos = 0;
                let mut base = bases[0];
                for &b in query {
                    pos = base.wrapping_add(i32::from(b)) as usize;
                    if let Some((_, check)) = out_checks.get(pos) {
                        if *check == b {
                            base = *bases.get_unchecked(pos);
                            continue;
                        }
                    }
                    return __TrieMatchValue::#wildcard_ident;
                }
                out_checks.get_unchecked(pos).0
            })( ::core::convert::AsRef::<[u8]>::as_ref( #expr ) ) {
                #( #arm, )*
            }
        }
    })
}

/// Generates a match expression that uses a trie structure.
///
/// # Examples
///
/// ```
/// use trie_match::trie_match;
///
/// let x = "abd";
///
/// trie_match! {
///     match x {
///         "a" => { println!("x"); }
///         "abc" => { println!("y"); }
///         "abd" | "bcc" => { println!("z"); }
///         "bc" => { println!("w"); }
///         _ => { println!(" "); }
///     }
/// }
/// ```
#[proc_macro]
pub fn trie_match(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ExprMatch);
    trie_match_inner(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
