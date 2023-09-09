use trie_match::trie_match;

#[test]
fn test_match() {
    let text = "abb";
    assert_eq!(
        trie_match!(match text {
            "abba" => 0,
            "abb" => 1,
            "ab" => 2,
            "" => 3,
            _ => 4,
        }),
        1,
    );
}

#[test]
fn test_match_empty() {
    let text = "";
    assert_eq!(
        trie_match!(match text {
            "abba" => 0,
            "abb" => 1,
            "ab" => 2,
            "" => 3,
            _ => 4,
        }),
        3,
    );
}

#[test]
fn test_match_wildcard() {
    let text = "ba";
    assert_eq!(
        trie_match!(match text {
            "abba" => 0,
            "abb" => 1,
            "ab" => 2,
            "" => 3,
            _ => 4,
        }),
        4,
    );
}
