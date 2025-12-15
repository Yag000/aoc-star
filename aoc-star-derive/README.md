# aoc-star-derive

Procedural macro crate for [`aoc-star`](https://crates.io/crates/aoc-star).

You normally don't use this crate directly. Instead, depend on `aoc-star`, and
import the `star` attribute from there:

```rust
use aoc_star::star;

#[star(day = 1, part = 1, year = 2024)]
fn solve_day1_part1(input: String) -> String {
    // ...
    "answer".to_string()
}
```

The `star` attribute:

- Registers the annotated function as an Advent of Code solution for a given
  `day`, `part`, and optional `year`.
- Ensures that the function has the correct signature: it must accept a single
  `String` argument (the puzzle input) and return a `String` (the answer).
- Uses the [`inventory`](https://crates.io/crates/inventory) crate to make the
  solution discoverable at runtime.
