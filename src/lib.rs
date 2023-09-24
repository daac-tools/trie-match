#![cfg_attr(feature = "cfg_attribute", feature(proc_macro_expand))]

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
//! let result = trie_match! {
//!     match x {
//!         "a" => 0,
//!         "abc" => 1,
//!         pat @ ("abd" | "bcde") => pat.len(),
//!         "bc" => 3,
//!         _ => 4,
//!     }
//! };
//!
//! assert_eq!(result, 3);
//! ```
#![cfg_attr(
    feature = "cfg_attribute",
    doc = r#"
## `cfg` attribute

Only when using Nightly Rust, this macro supports conditional compilation with
the `cfg` attribute. To use this feature, enable `features = ["cfg_attribute"]`
in your `Cargo.toml`.

### Example

```
use trie_match::trie_match;

let x = "abd";

let result = trie_match! {
    match x {
        #[cfg(not(feature = "foo"))]
        "a" => 0,
        "abc" => 1,
        #[cfg(feature = "bar")]
        "abd" | "bcc" => 2,
        "bc" => 3,
        _ => 4,
    }
};

assert_eq!(result, 4);
```
"#
)]
//!
//! ## Limitations
//!
//! The followings are different from the normal `match` expression:
//!
//! * Only supports strings, byte strings, and u8 slices as patterns.
//! * The wildcard is evaluated last. (The normal `match` expression does not
//!   match patterns after the wildcard.)
//! * Guards are unavailable.

mod trie;

extern crate proc_macro;

use std::collections::HashMap;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Arm, Error, Expr, ExprLit, ExprMatch, Lit, Pat, PatIdent,
    PatOr, PatReference, PatSlice, PatWild,
};

#[cfg(feature = "cfg_attribute")]
use proc_macro2::Ident;
#[cfg(feature = "cfg_attribute")]
use syn::{Attribute, Meta};

use crate::trie::Sparse;

static ERROR_UNEXPECTED_PATTERN: &str =
    "`trie_match` only supports string literals, byte string literals, and u8 slices as patterns";
static ERROR_ATTRIBUTE_NOT_SUPPORTED: &str = "attribute not supported here";
static ERROR_GUARD_NOT_SUPPORTED: &str = "match guard not supported";
static ERROR_UNREACHABLE_PATTERN: &str = "unreachable pattern";
static ERROR_PATTERN_NOT_COVERED: &str = "non-exhaustive patterns: `_` not covered";
static ERROR_EXPECTED_U8_LITERAL: &str = "expected `u8` integer literal";
static ERROR_VARIABLE_NOT_MATCH: &str = "variable is not bound in all patterns";

#[cfg(not(feature = "cfg_attribute"))]
static ERROR_ATTRIBUTE_NOT_SUPPORTED_CFG: &str =
    "attribute not supported here\nnote: consider enabling the `cfg_attribute` feature: \
    https://docs.rs/trie-match/latest/trie_match/#cfg-attribute";

#[cfg(feature = "cfg_attribute")]
static ERROR_NOT_CFG_ATTRIBUTE: &str = "only supports the cfg attribute";

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

struct PatternBytes {
    /// Bound variable identifier.
    ident: Option<PatIdent>,

    /// Byte sequence of this pattern. `None` is for a wildcard.
    bytes: Option<Vec<u8>>,
}

impl PatternBytes {
    const fn new(ident: Option<PatIdent>, bytes: Option<Vec<u8>>) -> Self {
        Self { ident, bytes }
    }
}

/// Retrieves pattern strings from the given token.
///
/// None indicates a wild card pattern (`_`).
fn retrieve_match_patterns(
    pat: &Pat,
    ident: Option<PatIdent>,
    pat_bytes_set: &mut Vec<PatternBytes>,
    pat_set: &mut Vec<Pat>,
) -> Result<(), Error> {
    match pat {
        Pat::Lit(lit) => {
            pat_set.push(pat.clone());
            pat_bytes_set.push(PatternBytes::new(ident, convert_literal_pattern(lit)?));
        }
        Pat::Slice(slice) => {
            pat_set.push(pat.clone());
            pat_bytes_set.push(PatternBytes::new(ident, convert_slice_pattern(slice)?));
        }
        Pat::Wild(pat) => {
            pat_bytes_set.push(PatternBytes::new(ident, convert_wildcard_pattern(pat)?));
        }
        Pat::Reference(reference) => {
            pat_set.push(pat.clone());
            pat_bytes_set.push(PatternBytes::new(
                ident,
                convert_reference_pattern(reference)?,
            ));
        }
        Pat::Ident(pat) => {
            if let Some(attr) = pat.attrs.first() {
                return Err(Error::new(attr.span(), ERROR_ATTRIBUTE_NOT_SUPPORTED));
            }
            let mut pat = pat.clone();
            if let Some((_, subpat)) = pat.subpat.take() {
                retrieve_match_patterns(&subpat, Some(pat), pat_bytes_set, pat_set)?;
            } else {
                pat_bytes_set.push(PatternBytes::new(Some(pat), None));
            }
        }
        Pat::Paren(pat) => {
            retrieve_match_patterns(&pat.pat, ident, pat_bytes_set, pat_set)?;
        }
        Pat::Or(PatOr {
            attrs,
            leading_vert: None,
            cases,
        }) => {
            if let Some(attr) = attrs.first() {
                return Err(Error::new(attr.span(), ERROR_ATTRIBUTE_NOT_SUPPORTED));
            }
            for pat in cases {
                retrieve_match_patterns(pat, ident.clone(), pat_bytes_set, pat_set)?;
            }
        }
        _ => {
            return Err(Error::new(pat.span(), ERROR_UNEXPECTED_PATTERN));
        }
    }
    Ok(())
}

#[cfg(feature = "cfg_attribute")]
fn evaluate_cfg_attribute(attrs: &[Attribute]) -> Result<bool, Error> {
    for attr in attrs {
        let ident = attr.path().get_ident().map(Ident::to_string);
        if ident.as_deref() == Some("cfg") {
            if let Meta::List(list) = &attr.meta {
                let tokens = &list.tokens;
                let cfg_macro: proc_macro::TokenStream = quote! { cfg!(#tokens) }.into();
                let expr = cfg_macro
                    .expand_expr()
                    .map_err(|e| Error::new(tokens.span(), e.to_string()))?;
                if expr.to_string() == "false" {
                    return Ok(false);
                }
                continue;
            }
        }
        return Err(Error::new(attr.span(), ERROR_NOT_CFG_ATTRIBUTE));
    }
    Ok(true)
}

struct MatchInfo {
    bodies: Vec<Expr>,
    pattern_map: HashMap<Vec<u8>, usize>,
    wildcard_idx: usize,
    bound_vals: Vec<Option<PatIdent>>,
    pat_set: Vec<Pat>,
}

fn parse_match_arms(arms: Vec<Arm>) -> Result<MatchInfo, Error> {
    let mut pattern_map = HashMap::new();
    let mut wildcard_idx = None;
    let mut bound_vals = vec![];
    let mut bodies = vec![];
    let mut pat_set = vec![];
    let mut i = 0;
    #[allow(clippy::explicit_counter_loop)]
    for Arm {
        attrs,
        pat,
        guard,
        body,
        ..
    } in arms
    {
        #[cfg(feature = "cfg_attribute")]
        if !evaluate_cfg_attribute(&attrs)? {
            continue;
        }
        #[cfg(not(feature = "cfg_attribute"))]
        if let Some(attr) = attrs.first() {
            return Err(Error::new(attr.span(), ERROR_ATTRIBUTE_NOT_SUPPORTED_CFG));
        }

        if let Some((if_token, _)) = guard {
            return Err(Error::new(if_token.span(), ERROR_GUARD_NOT_SUPPORTED));
        }
        let mut pat_bytes_set = vec![];
        retrieve_match_patterns(&pat, None, &mut pat_bytes_set, &mut pat_set)?;
        let bound_val = pat_bytes_set[0].ident.clone();
        for PatternBytes { ident, bytes } in pat_bytes_set {
            if ident != bound_val {
                return Err(Error::new(
                    ident.or(bound_val).unwrap().span(),
                    ERROR_VARIABLE_NOT_MATCH,
                ));
            }
            if let Some(bytes) = bytes {
                if pattern_map.contains_key(&bytes) {
                    return Err(Error::new(pat.span(), ERROR_UNREACHABLE_PATTERN));
                }
                pattern_map.insert(bytes, i);
            } else {
                if wildcard_idx.is_some() {
                    return Err(Error::new(pat.span(), ERROR_UNREACHABLE_PATTERN));
                }
                wildcard_idx.replace(i);
            }
        }
        bound_vals.push(bound_val);
        bodies.push(*body);
        i += 1;
    }
    let Some(wildcard_idx) = wildcard_idx else {
        return Err(Error::new(Span::call_site(), ERROR_PATTERN_NOT_COVERED));
    };
    Ok(MatchInfo {
        bodies,
        pattern_map,
        wildcard_idx,
        bound_vals,
        pat_set,
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
        bound_vals,
        pat_set,
    } = parse_match_arms(arms)?;
    let mut trie = Sparse::new();
    for (k, v) in pattern_map {
        if v == wildcard_idx {
            continue;
        }
        trie.add(k, v);
    }
    let (bases, checks, outs) = trie.build_double_array_trie(wildcard_idx);

    let out_check = outs.iter().zip(checks).map(|(out, check)| {
        let out = format_ident!("V{out}");
        quote! { (__TrieMatchValue::#out, #check) }
    });
    let arm = bodies
        .iter()
        .zip(bound_vals)
        .enumerate()
        .map(|(i, (body, bound_val))| {
            let i = format_ident!("V{i}");
            let bound_val = bound_val.map_or_else(|| quote! { _ }, |val| quote! { #val });
            quote! { (__TrieMatchValue::#i, #bound_val ) => #body }
        });
    let enumvalue = (0..bodies.len()).map(|i| format_ident!("V{i}"));
    let wildcard_ident = format_ident!("V{wildcard_idx}");
    Ok(quote! {
        {
            #[derive(Clone, Copy)]
            enum __TrieMatchValue {
                #( #enumvalue, )*
            }
            #( #attrs )*
            match #expr {
                // This is for type inference.
                query @ ( #( #pat_set | )* _) => {
                    match (|query| unsafe {
                        let query_ref = ::core::convert::AsRef::<[u8]>::as_ref(&query);
                        let bases: &'static [i32] = &[ #( #bases, )* ];
                        let out_checks: &'static [(__TrieMatchValue, u8)] = &[ #( #out_check, )* ];
                        let mut pos = 0;
                        let mut base = bases[0];
                        for &b in query_ref {
                            pos = base.wrapping_add(i32::from(b)) as usize;
                            if let Some((_, check)) = out_checks.get(pos) {
                                if *check == b {
                                    base = *bases.get_unchecked(pos);
                                    continue;
                                }
                            }
                            return (__TrieMatchValue::#wildcard_ident, query);
                        }
                        (out_checks.get_unchecked(pos).0, query)
                    })(query) {
                        #( #arm, )*
                    }
                }
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
/// let result = trie_match! {
///     match x {
///         "a" => 0,
///         "abc" => 1,
///         pat @ ("abd" | "bcde") => pat.len(),
///         "bc" => 3,
///         _ => 4,
///     }
/// };
///
/// assert_eq!(result, 3);
/// ```
#[proc_macro]
pub fn trie_match(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as ExprMatch);
    trie_match_inner(input)
        .unwrap_or_else(Error::into_compile_error)
        .into()
}
