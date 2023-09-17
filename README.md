# `trie_match! {}`

[![Crates.io](https://img.shields.io/crates/v/trie-match)](https://crates.io/crates/trie-match)
[![Documentation](https://docs.rs/trie-match/badge.svg)](https://docs.rs/trie-match)
[![Rust](https://img.shields.io/badge/rust-1.60%2B-blue.svg?maxAge=3600)](https://github.com/daac-tools/trie-match)
[![Build Status](https://github.com/daac-tools/trie-match/actions/workflows/rust.yml/badge.svg)](https://github.com/daac-tools/trie-match/actions)
[![Slack](https://img.shields.io/badge/join-chat-brightgreen?logo=slack)](https://join.slack.com/t/daac-tools/shared_invite/zt-1pwwqbcz4-KxL95Nam9VinpPlzUpEGyA)

This macro speeds up Rust's `match` expression for comparing strings by using a
compact double-array data structure.

## Usage

Simply wrap the existing `match` expression with the `trie_match! {}` macro as
follows:

```rust
use trie_match::trie_match;

let x = "abd";

trie_match! {
    match x {
        "a" => { println!("x"); }
        "abc" => { println!("y"); }
        "abd" | "bcc" => { println!("z"); }
        "bc" => { println!("w"); }
        _ => { println!(" "); }
    }
}
```

## Why is it faster?

In a normal `match` expression, the string is compared for each pattern. It is
equivalent to the following code:

```rust
if x == "a" {
    ..
} else if x == "abc" {
    ..
} else if x == "abd" || x == "bcc" {
    ..
} else if x == "bc" {
    ..
} else {
    ..
}
```

The above code requires that string comparisons be made from the beginning of
the string each time. The time complexity of the above code is *O(mn)*, where
*m* is the average pattern length, and *n* is the number of patterns.

In contrast, this macro builds the following trie structure to retrieve the
index of the matched arm:

![Trie](figures/graph.svg)

Furthermore, this implementation uses the compact double-array data structure
to achieve efficient state-to-state traversal, and the time complexity becomes
*O(m)*.

## Limitations

The followings are different from the normal `match` expression:

* Only supports strings, byte strings, and u8 slices as patterns.
* The wildcard is evaluated last. (The normal `match` expression does not
  match patterns after the wildcard.)
* Pattern bindings are unavailable.
* Attributes for match arms are unavailable.
* Guards are unavailable.

Sometimes the normal `match` expression is faster, depending on how
optimization is performed, so it is better to choose based on your speed
experiments.

## Benchmark

Run the following command:

```
cargo bench
```

Experimental results are as follows [μs]:

* AMD Ryzen 7 5700U with Radeon Graphics

  | Bench name           | Normal match | Trie match |
  |----------------------|--------------|------------|
  | 100 words random     |         1.97 |   **1.01** |
  | HTML elements random |         2.20 |   **0.55** |

* 12th Gen Intel(R) Core(TM) i7-1270P

  | Bench name           | Normal match | Trie match |
  |----------------------|--------------|------------|
  | 100 words random     |         1.63 |   **0.84** |
  | HTML elements random |         1.64 |   **0.48** |

## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

See [the guidelines](./CONTRIBUTING.md).
