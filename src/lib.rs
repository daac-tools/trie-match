//! # `trie_match! {}`
//!
//! This macro speeds up a `match` expression comparing strings by the compact double-array data
//! structure.
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
use quote::quote;
use syn::{
    parse_macro_input, spanned::Spanned, Arm, Error, ExprLit, ExprMatch, Lit, Pat, PatOr, PatWild,
};

use crate::trie::Sparse;

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

fn trie_match_inner(input: ExprMatch) -> Result<TokenStream, Error> {
    let ExprMatch {
        attrs, expr, arms, ..
    } = input;
    let mut map = HashMap::new();
    let mut wildcard_idx = None;
    let mut built_arms = vec![];
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
            return Err(Error::new(attr.span(), "attribute not supported here"));
        }
        if let Some((if_token, _)) = guard {
            return Err(Error::new(if_token.span(), "match guard not supported"));
        }
        let pat_strs = retrieve_match_patterns(&pat)?;
        let i = u32::try_from(i).unwrap();
        for pat_str in pat_strs {
            if let Some(pat_str) = pat_str {
                if map.contains_key(&pat_str) {
                    return Err(Error::new(pat.span(), "unreachable pattern"));
                }
                map.insert(pat_str, i);
            } else {
                if wildcard_idx.is_some() {
                    return Err(Error::new(pat.span(), "unreachable pattern"));
                }
                wildcard_idx.replace(i);
            }
        }
        built_arms.push(quote! { #i => #body });
    }
    if wildcard_idx.is_none() {
        return Err(Error::new(
            Span::call_site(),
            "non-exhaustive patterns: `_` not covered",
        ));
    }
    let wildcard_idx = wildcard_idx.unwrap();
    let mut trie = Sparse::new();
    for (k, v) in map {
        trie.add(k, v);
    }
    let (bases, out_checks) = trie.build_double_array_trie();

    let base = bases.iter();
    let out_check = out_checks.iter();
    let arm = built_arms.iter();
    let attr = attrs.iter();
    Ok(quote! {
        #( #attr )*
        match (|query: &str| unsafe {
            let bases: &'static [i32] = &[ #( #base, )* ];
            let out_checks: &'static [u32] = &[ #( #out_check, )* ];
            let mut pos = 0;
            for &b in query.as_bytes() {
                let base = *bases.get_unchecked(pos);
                pos = base.wrapping_add(i32::from(b)) as usize;
                if let Some(out_check) = out_checks.get(pos) {
                    if out_check & 0xff == u32::from(b) {
                        continue;
                    }
                }
                return #wildcard_idx;
            }
            let out = *out_checks.get_unchecked(pos) >> 8;
            if out != 0xffffff {
                out
            } else {
                #wildcard_idx
            }
        })( #expr ) {
            #( #arm, )*
            _ => unreachable!(),
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
