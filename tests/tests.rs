#![no_std]

use trie_match::trie_match;

#[test]
fn test_only_wildcard() {
    let f = |text| {
        trie_match! {
            match text {
                _ => 4,
            }
        }
    };
    assert_eq!(f(""), 4);
    assert_eq!(f("a"), 4);
    assert_eq!(f("ab"), 4);
}

#[test]
fn test_prefix_patterns() {
    // 0 -a-> 1 -b-> 2 -c-> * -d-> 3
    let f = |text| {
        trie_match! {
            match text {
                "" => 0,
                "a" => 1,
                "ab" => 2,
                "abcd" => 3,
                _ => 4,
            }
        }
    };
    assert_eq!(f(""), 0);
    assert_eq!(f("a"), 1);
    assert_eq!(f("ab"), 2);
    assert_eq!(f("abc"), 4);
    assert_eq!(f("abcd"), 3);
    assert_eq!(f("b"), 4);
}

#[test]
fn test_longer_query() {
    // * -a-> * -b-> 0
    let f = |text| {
        trie_match! {
            match text {
                "ab" => 0,
                _ => 1,
            }
        }
    };
    assert_eq!(f("ab"), 0);
    assert_eq!(f("abcdefg"), 1);
}

#[test]
fn test_branch_root() {
    // * -a-> 0
    //  \
    //   \-b-> 1
    let f = |text| {
        trie_match! {
            match text {
                "a" => 0,
                "b" => 1,
                _ => 2,
            }
        }
    };
    assert_eq!(f("a"), 0);
    assert_eq!(f("b"), 1);
    assert_eq!(f("c"), 2);
}

#[test]
fn test_branch_multiple_times() {
    //                     /-e-> 5
    //                    /
    // * --a--> 0 --b--> * --c--> * --d--> 1
    //  \        \        \
    //   \-b-> 2  \-c-> 3  \-d-> * --e--> 4
    let f = |text| {
        trie_match! {
            match text {
                "a" => 0,
                "abcd" => 1,
                "b" => 2,
                "ac" => 3,
                "abde" => 4,
                "abe" => 5,
                _ => 6,
            }
        }
    };
    assert_eq!(f(""), 6);
    assert_eq!(f("a"), 0);
    assert_eq!(f("ab"), 6);
    assert_eq!(f("abc"), 6);
    assert_eq!(f("abcd"), 1);
    assert_eq!(f("abd"), 6);
    assert_eq!(f("abde"), 4);
    assert_eq!(f("abe"), 5);
    assert_eq!(f("ac"), 3);
    assert_eq!(f("b"), 2);
    assert_eq!(f("abcde"), 6);
    assert_eq!(f("abdef"), 6);
    assert_eq!(f("acd"), 6);
    assert_eq!(f("ad"), 6);
    assert_eq!(f("bc"), 6);
    assert_eq!(f("c"), 6);
}

// This test confirms that the generator prevents base value conflictions.
#[test]
fn test_try_base_conflict() {
    let f = |text| {
        trie_match! {
            match text {
                // The following pattern adds multiple zeros into a base array in a normal
                // double-array, but it is not allowed in a compact double-array.
                "\u{1}\u{2}\u{3}" => 0,
                _ => 1,
            }
        }
    };
    assert_eq!(f("\u{1}\u{2}\u{3}"), 0);
    assert_eq!(f("\u{2}\u{3}"), 1);
    assert_eq!(f("\u{3}"), 1);
}

// This test confirms that check[0] does not have an invalid value of zero.
#[test]
fn test_invalid_root_check_of_zero() {
    // [0] -x01-> [1]
    //    \-x00-> [0] ? If check[0] is 0, such an invalid transition is possible.
    //
    //  base: [0, MAX]
    // check: [0,   1]
    let f = |text| {
        trie_match! {
            match text {
                "\u{1}" => 1,
                _ => 0,
            }
        }
    };
    assert_eq!(f("\u{0}\u{1}"), 0);
}

#[test]
fn test_bytes_literal() {
    let f = |text: &[u8]| {
        trie_match! {
            match text {
                b"abc" => 0,
                _ => 1,
            }
        }
    };
    assert_eq!(f(b"abc"), 0);
    assert_eq!(f(b"ab"), 1);
}

#[test]
fn test_slice_byte_literal() {
    let f = |text: &[u8]| {
        trie_match! {
            match text {
                [b'a', b'b', b'c'] => 0,
                _ => 1,
            }
        }
    };
    assert_eq!(f(b"abc"), 0);
    assert_eq!(f(b"ab"), 1);
}

#[test]
fn test_slice_numbers() {
    let f = |text: &[u8]| {
        trie_match! {
            match text {
                [0, 1, 2] => 0,
                _ => 1,
            }
        }
    };
    assert_eq!(f(&[0, 1, 2]), 0);
    assert_eq!(f(&[0, 1]), 1);
}

#[test]
fn test_slice_ref_numbers() {
    let f = |text: &[u8]| {
        trie_match! {
            match text {
                &[0, 1, 2] => 0,
                _ => 1,
            }
        }
    };
    assert_eq!(f(&[0, 1, 2]), 0);
    assert_eq!(f(&[0, 1]), 1);
}

#[cfg(feature = "cfg_attribute")]
#[test]
fn test_cfg_attribute() {
    let f = |text| {
        trie_match! {
            match text {
                #[cfg(test)]
                "a" => 0,
                #[cfg(not(test))]
                "b" => 1,
                _ => 2,
            }
        }
    };
    assert_eq!(f("a"), 0);
    assert_eq!(f("b"), 2);
    assert_eq!(f("c"), 2);
}

#[cfg(feature = "cfg_attribute")]
#[test]
fn test_cfg_attribute_combination() {
    let f = |text| {
        trie_match! {
            match text {
                #[cfg(test)]
                #[cfg(feature = "cfg_attribute")]
                "a" => 0,
                #[cfg(not(test))]
                #[cfg(feature = "cfg_attribute")]
                "b" => 1,
                #[cfg(test)]
                #[cfg(not(feature = "cfg_attribute"))]
                "c" => 2,
                #[cfg(not(test))]
                #[cfg(not(feature = "cfg_attribute"))]
                "d" => 3,
                _ => 4,
            }
        }
    };
    assert_eq!(f("a"), 0);
    assert_eq!(f("b"), 4);
    assert_eq!(f("c"), 4);
    assert_eq!(f("d"), 4);
}
