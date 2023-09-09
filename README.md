# `trie_match!()`

## Usage

```rust
trie_match!(
    match x {
        "abc" => { .. }
        "acd" | "bcc" => { .. }
        "ba" => { .. }
        _ => { .. }
    }
)
```
