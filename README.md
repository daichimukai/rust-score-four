# Score Four Library for Rust
This is a simple Score Four board library for Rust.

## Example
```rust
extern crate score_four;
use score_four::{Board, BoardStatus};

let mut board = Board::new();
board.put(1); // put a bead in (0, 0) position
assert_eq!(board.status(), BoardStatus::Ongoing);
```
