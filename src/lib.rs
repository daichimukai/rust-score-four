//! # Score Four Library for Rust
//! This is a simple Score Four board library for Rust.
//!
//! ## Example
//! ```
//! extern crate score_four;
//! use score_four::{Board, BoardStatus};
//!
//! let mut board = Board::new();
//! board.put(0); // put a bead in (0, 0) position
//! assert_eq!(board.status(), BoardStatus::Ongoing);
//! ```
//!

extern crate rand;

mod bitboard;
pub use bitboard::{BitBoard, EMPTY};

mod color;
pub use color::{Color, NUM_COLORS};

mod board;
pub use board::{Board, BoardStatus};

mod action;
pub use action::Action;

mod player;
pub use player::{Player, PlayerMC, PlayerRandom};

mod organizer;
pub use organizer::Organizer;
