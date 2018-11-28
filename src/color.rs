use std::ops::Not;

/// Colors of players to play
#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
pub enum Color {
    White,
    Black,
}

/// number of colors
pub const NUM_COLORS: usize = 2;

impl Color {
    pub fn to_index(&self) -> usize {
        *self as usize
    }
}

impl Not for Color {
    type Output = Color;

    fn not(self) -> Color {
        match self {
            Color::White => Color::Black,
            _ => Color::White,
        }
    }
}
