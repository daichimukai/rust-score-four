use bitboard::*;
use color::*;

/// A representation of a score four board
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Board {
    beads: [BitBoard; NUM_COLORS],
    combined: BitBoard,
    side_to_move: Color,
}

/// Status of a board, playing or lines are made
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum BoardStatus {
    Ongoing,
    WhiteWin,
    BlackWin,
    Draw,
}

impl Board {
    /// Construct a new board
    ///
    /// ```
    /// use score_four::{Board, EMPTY};
    ///
    /// let board = Board::new();
    ///
    /// assert_eq!(board.combined(), EMPTY);
    /// ```
    ///
    pub fn new() -> Board {
        Board {
            beads: [EMPTY; NUM_COLORS],
            combined: EMPTY,
            side_to_move: Color::White,
        }
    }

    /// Combined bitboard of two colors
    /// ```
    /// use score_four::{Board, EMPTY};
    ///
    /// let board = Board::new();
    ///
    /// assert_eq!(board.combined(), EMPTY);
    /// ```
    ///
    pub fn combined(&self) -> BitBoard {
        self.combined
    }

    /// Who's turn?
    ///
    /// ```
    /// use score_four::{Board, Color};
    ///
    /// let mut board = Board::new();
    ///
    /// assert_eq!(board.side_to_move(), Color::White);
    ///
    /// board.put(16);
    ///
    /// assert_eq!(board.side_to_move(), Color::Black);
    ///
    /// ```
    ///
    pub fn side_to_move(&self) -> Color {
        self.side_to_move
    }

    /// Put a bead at the given position
    /// ```
    /// use score_four::{Board, Color, BitBoard, EMPTY};
    ///
    /// let mut board = Board::new();
    ///
    /// board.put(1);
    /// ```
    ///
    pub fn put(&mut self, pos: u8) {
        let color = self.side_to_move.to_index();
        let put_pos = BitBoard::new((pos as u64) << (16 * self.beads[color].get_level_at(pos)));

        assert_eq!(put_pos.0.count_ones(), 1);

        self.beads[color] |= put_pos;
        self.combined = self.beads[0] | self.beads[1];
        self.side_to_move = !self.side_to_move;
    }

    /// Put a bead at the given position
    /// ```
    /// use score_four::{Board, BoardStatus};
    ///
    /// let mut board = Board::new();
    ///
    /// assert_eq!(board.status(), BoardStatus::Ongoing);
    ///
    /// board.put(1);
    /// board.put(2);
    /// board.put(1);
    /// board.put(2);
    /// board.put(1);
    /// board.put(2);
    /// board.put(1);
    ///
    /// assert_eq!(board.status(), BoardStatus::WhiteWin);
    /// ```
    ///
    pub fn status(&self) -> BoardStatus {
        if self.beads[Color::White.to_index()].lined() {
            return BoardStatus::WhiteWin;
        }

        if self.beads[Color::Black.to_index()].lined() {
            return BoardStatus::BlackWin;
        }

        if self.combined.popcnt() == 64 {
            return BoardStatus::Draw;
        }

        BoardStatus::Ongoing
    }
}
