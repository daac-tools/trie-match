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
//! * Only supports string comparison.
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
    PatWild,
};

use crate::trie::Sparse;

/// Retrieves pattern strings from the given token.
///
/// None indicates a wild card pattern (`_`).
fn retrieve_match_patterns(pat: &Pat) -> Result<Vec<Option<String>>, Error> {
    let mut pats = vec![];
    match pat {
        Pat::Lit(ExprLit {
            lit: Lit::Str(s),
            attrs,
        }) => {
            if let Some(attr) = attrs.first() {
                return Err(Error::new(attr.span(), "attribute not supported here"));
            }
            pats.push(Some(s.value()));
        }
        Pat::Or(PatOr {
            attrs,
            leading_vert: None,
            cases,
        }) => {
            if let Some(attr) = attrs.first() {
                return Err(Error::new(attr.span(), "attribute not supported here"));
            }
            for pat in cases {
                match pat {
                    Pat::Lit(ExprLit {
                        lit: Lit::Str(s),
                        attrs,
                    }) => {
                        if let Some(attr) = attrs.first() {
                            return Err(Error::new(attr.span(), "attribute not supported here"));
                        }
                        pats.push(Some(s.value()));
                    }
                    _ => {
                        return Err(Error::new(
                            pat.span(),
                            "`trie_match` only supports string literal patterns",
                        ));
                    }
                }
            }
        }
        Pat::Wild(PatWild { attrs, .. }) => {
            if let Some(attr) = attrs.first() {
                return Err(Error::new(attr.span(), "attribute not supported here"));
            }
            pats.push(None);
        }
        _ => {
            return Err(Error::new(
                pat.span(),
                "`trie_match` only supports string literal patterns",
            ));
        }
    }
    Ok(pats)
}

struct MatchInfo {
    bodies: Vec<Expr>,
    pattern_map: HashMap<String, usize>,
    wildcard_idx: usize,
}

fn parse_match_arms(arms: &[Arm]) -> Result<MatchInfo, Error> {
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
    ) in arms.iter().enumerate()
    {
        if let Some(attr) = attrs.first() {
            return Err(Error::new(attr.span(), "attribute not supported here"));
        }
        if let Some((if_token, _)) = guard {
            return Err(Error::new(if_token.span(), "match guard not supported"));
        }
        let pat_strs = retrieve_match_patterns(pat)?;
        for pat_str in pat_strs {
            if let Some(pat_str) = pat_str {
                if pattern_map.contains_key(&pat_str) {
                    return Err(Error::new(pat.span(), "unreachable pattern"));
                }
                pattern_map.insert(pat_str, i);
            } else {
                if wildcard_idx.is_some() {
                    return Err(Error::new(pat.span(), "unreachable pattern"));
                }
                wildcard_idx.replace(i);
            }
        }
        bodies.push(*body.clone());
    }
    let Some(wildcard_idx) = wildcard_idx else {
        return Err(Error::new(
            Span::call_site(),
            "non-exhaustive patterns: `_` not covered",
        ));
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
    } = parse_match_arms(&arms)?;

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
            match (|query: &str| unsafe {
                let bases: &'static [i32] = &[ #( #base, )* ];
                let out_checks: &'static [(__TrieMatchValue, u8)] = &[ #( #out_check, )* ];
                let mut pos = 0;
                let mut base = bases[0];
                for &b in query.as_bytes() {
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
            })( #expr ) {
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
