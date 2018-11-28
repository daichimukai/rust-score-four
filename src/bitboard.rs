use std::ops::{BitAnd, BitOr, BitAndAssign, BitOrAssign, Not};

/// A bit-board for Score Four
///
/// ```
/// use score_four::BitBoard;
///
/// let b = BitBoard::new(0b1000_1000_1000_1000); // 4 beads in the most lower level
///
/// assert_eq!(b.popcnt(), 4);
/// ```
///
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug, Default)]
pub struct BitBoard(pub u64);

/// An empty bitboard.
///
///
/// ```
/// use score_four::EMPTY;
///
/// assert_eq!(EMPTY.popcnt(), 0);
/// assert_eq!((!EMPTY).popcnt(), 64);
/// ```
///
pub const EMPTY: BitBoard = BitBoard(0);

macro_rules! from_each_level {
    ($arr: expr) => ($arr[0] | ($arr[1] << 16) | ($arr[2] << 32) | ($arr[3] << 48))
}

impl BitBoard {
    /// Construct a new bitboard instance from u64
    pub fn new(b: u64) -> BitBoard {
        BitBoard(b)
    }

    /// Construct a new bitboard representing a line at the given position
    ///
    /// ```
    /// use score_four::BitBoard;
    ///
    /// let bb = BitBoard::line_at_pos(1);
    ///
    /// assert_eq!(bb, BitBoard(281_4792_7174_3489));
    /// ```
    ///
    pub fn line_at_pos(_pos: u8) -> BitBoard {
        let pos = _pos as u64;
        BitBoard(pos | (pos << 16) | (pos << 32) | (pos << 48))
    }

    /// Get the level at the given position
    ///
    /// ```
    /// use score_four::BitBoard;
    ///
    /// let bb = BitBoard::line_at_pos(1);
    ///
    /// assert_eq!(bb.get_level_at(1), 4);
    /// ```
    ///
    pub fn get_level_at(&self, pos: u8) -> u8 {
        (*self & BitBoard::line_at_pos(pos)).0.count_ones() as u8
    }

    /// Count the numbers of beads in this bitboard
    ///
    /// ```
    /// use score_four::BitBoard;
    ///
    /// let bb = BitBoard::new(u64::max_value());
    ///
    /// assert_eq!(bb.popcnt(), 64);
    /// ```
    ///
    pub fn popcnt(&self) -> u32 {
        self.0.count_ones()
    }

    /// Check if a line has constructed
    ///
    /// ```
    /// use score_four::BitBoard;
    ///
    /// let bb = BitBoard::new(0b1111);
    ///
    /// assert!(bb.lined());
    /// ```
    ///
    pub fn lined(&self) -> bool {
        self.row_lined() || self.column_lined() || self.level_lined() || self.diagonal_lined()
    }

    /// Check there exists a lined row or not
    ///
    /// ```
    /// use score_four::BitBoard;
    ///
    /// let bb = BitBoard::new(0b1111);
    ///
    /// assert_eq!(bb.row_lined(), true);
    ///
    /// let bb = BitBoard::new(0b0001_0001_0001_0001);
    ///
    /// assert_eq!(bb.row_lined(), false);
    /// ```
    ///
    pub fn row_lined(&self) -> bool {
        for level in 0..4 {
            for column in 0..4 {
                let mask = BitBoard(0b1111 << (4 * column + 16 * level));
                if mask == (*self & mask) {
                    return true
                }
            }
        }

        false
    }

    /// Check there exists a lined column or not
    ///
    /// ```
    /// use score_four::BitBoard;
    ///
    /// let bb = BitBoard::new(0b0001_0001_0001_0001);
    ///
    /// assert_eq!(bb.column_lined(), true);
    ///
    /// let bb = BitBoard::new(0b1111);
    ///
    /// assert_eq!(bb.column_lined(), false);
    /// ```
    ///
    pub fn column_lined(&self) -> bool {
        for level in 0..4 {
            for row in 0..4 {
                let mask = BitBoard(0b0001_0001_0001_0001 << (row + 16 * level));
                if mask == (*self & mask) {
                    return true
                }
            }
        }

        false
    }

    /// Check there exists a lined column or not
    ///
    /// ```
    /// use score_four::BitBoard;
    ///
    /// let bb = BitBoard::line_at_pos(1);
    ///
    /// assert_eq!(bb.level_lined(), true);
    ///
    /// let bb = BitBoard::new(0b1111);
    ///
    /// assert_eq!(bb.level_lined(), false);
    /// ```
    ///
    pub fn level_lined(&self) -> bool {
        let line_tmpl = 0b0001 + (0b0001 << 16) + (0b0001 << 32) + (0b0001 << 48);
        for row in 0..4 {
            for column in 0..4 {
                let mask = BitBoard(line_tmpl << (row + 4 * column));
                if mask == (*self & mask) {
                    return true
                }
            }
        }

        false
    }

    /// Check there exists a lined diagonal or not
    fn diagonal_2d_lined(&self) -> bool {
        let line_tmpl = [0b1000_0100_0010_0001, 0b0001_0010_0100_1000];
        for level in 0..4 {
            for i in 0..2 {
                let mask = BitBoard(line_tmpl[i] << (16 * level));
                if mask == (*self & mask) {
                    return true
                }
            }
        }

        let line_tmpl = [from_each_level!([0b0000_0000_0000_0001, 0b0000_0000_0000_0010,
                                           0b0000_0000_0000_0100, 0b0000_0000_0000_1000]),
                         from_each_level!([0b0000_0000_0000_1000, 0b0000_0000_0000_0100,
                                           0b0000_0000_0000_0010, 0b0000_0000_0000_0001])];
        for row in 0..4 {
            for i in 0..2 {
                let mask = BitBoard(line_tmpl[i] << (4 * row));
                if mask == (*self & mask) {
                    return true
                }
            }
        }

        let line_tmpl = [from_each_level!([0b0000_0000_0000_0001, 0b0000_0000_0001_0000,
                                           0b0000_0001_0000_0000, 0b0001_0000_0000_0000]),
                         from_each_level!([0b0001_0000_0000_0000, 0b0000_0001_0000_0000,
                                           0b0000_0000_0001_0000, 0b0000_0000_0000_0001])];
        for column in 0..4 {
            for i in 0..2 {
                let mask = BitBoard(line_tmpl[i] << column);
                if mask == (*self & mask) {
                    return true
                }
            }
        }

        false
    }

    /// Check there exists a lined diagonal or not
    fn diagonal_3d_lined(&self) -> bool {
        let line_tmpl = [from_each_level!([0b0000_0000_0000_0001, 0b0000_0000_0010_0000,
                                           0b0000_0100_0000_0000, 0b1000_0000_0000_0000]),
                         from_each_level!([0b1000_0000_0000_0000, 0b0000_0100_0000_0000,
                                           0b0000_0000_0010_0000, 0b0000_0000_0000_0001]),
                         from_each_level!([0b0000_0000_0000_1000, 0b0000_0000_0100_0000,
                                           0b0000_0010_0000_0000, 0b0001_0000_0000_0000]),
                         from_each_level!([0b0001_0000_0000_0000, 0b0000_0010_0000_0000,
                                           0b0000_0000_0100_0000, 0b0000_0000_0000_1000]),];
        for _mask in line_tmpl.iter() {
            let mask = BitBoard(*_mask);
            if mask == (*self & mask) {
                return true
            }
        }

        false
    }

    /// Check there exists a lined diagonal or not
    fn diagonal_lined(&self) -> bool {
        self.diagonal_2d_lined() || self.diagonal_3d_lined()
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 & other.0)
    }
}

impl BitOr for BitBoard {
    type Output = BitBoard;

    fn bitor(self, other: BitBoard) -> BitBoard {
        BitBoard(self.0 | other.0)
    }
}

impl BitAndAssign for BitBoard {
    fn bitand_assign(&mut self, other: BitBoard) {
        self.0 &= other.0
    }
}

impl BitOrAssign for BitBoard {
    fn bitor_assign(&mut self, other: BitBoard) {
        self.0 |= other.0
    }
}

impl Not for BitBoard {
    type Output = BitBoard;

    fn not(self) -> BitBoard {
        BitBoard(!self.0)
    }
}
