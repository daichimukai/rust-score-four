use action::*;
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
    /// board.put(0);
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
    pub fn put(&mut self, pos: u8) -> Result<(u8), String> {
        if pos > 15 {
            return Err(format!(
                "The position should be given by one of 0, 1,..., 15 (got {})",
                pos
            ));
        }

        let color = self.side_to_move.to_index();
        let level = self.combined.get_level_at(pos);

        if level > 3 {
            return Err(format!("The bar at {} already has 4 beads.", pos));
        }

        let pos_bit = 0b1 << pos;
        let put_pos = BitBoard::new(pos_bit << (16 * level));

        self.beads[color] |= put_pos;
        self.combined = self.beads[0] | self.beads[1];
        self.side_to_move = !self.side_to_move;

        Ok(level + 1)
    }

    /// Put a bead at the position given by action.
    ///
    /// ```
    /// use score_four::{Board};
    ///
    /// let mut board = Board::new();
    /// let actions = board.possible_actions();
    /// board.step(actions[0]);
    /// ```
    pub fn step(&mut self, action: Action) -> Result<(u8), String> {
        self.put(action.position())
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

    /// Return possible actions.
    ///
    /// ```
    /// use score_four::Board;
    ///
    /// let mut board = Board::new();
    /// let actions = board.possible_actions();
    ///
    /// assert_eq!(actions.len(), 16);
    /// ```
    pub fn possible_actions(&self) -> Vec<Action> {
        let mut ret = vec![];
        let combined = self.combined.clone();
        for pos in 0..16 {
            if combined.get_level_at(pos) < 4 {
                ret.push(Action::new(pos));
            }
        }

        ret
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_board_put() {
        let mut b = Board::new();

        assert_eq!(b.put(0).unwrap(), 1);
        assert_eq!(b.put(0).unwrap(), 2);
        assert_eq!(b.put(0).unwrap(), 3);
        assert_eq!(b.put(0).unwrap(), 4);

        assert_eq!(b.combined().popcnt(), 4);
        assert_eq!(b.side_to_move, Color::White);
        assert!(b.put(0).is_err());
        assert_eq!(b.side_to_move, Color::White);

        assert_eq!(b.put(1).unwrap(), 1);
        assert_eq!(b.put(2).unwrap(), 1);
        assert_eq!(b.put(3).unwrap(), 1);
        assert_eq!(b.put(4).unwrap(), 1);

        assert_eq!(b.combined().popcnt(), 8);

        assert!(b.put(16).is_err());
    }
}
