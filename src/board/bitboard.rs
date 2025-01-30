use std::{arch::x86_64, num::{NonZeroU64, NonZeroUsize}, ops::{BitAnd, BitOr, Deref, Sub}, path::Iter};

use super::square::Square;


#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BitBoard(pub u64);

impl BitBoard {
    /// An empty BitBoard.
    pub const EMPTY: BitBoard = BitBoard(0);

    /// A full BitBoard.
    pub const FULL: BitBoard = BitBoard(0xFFFFFFFFFFFFFFFF);

    // The ranks of the board.
    pub const RANK_1: BitBoard = BitBoard(0xFF);
    pub const RANK_2: BitBoard = BitBoard(0xFF00);
    pub const RANK_3: BitBoard = BitBoard(0xFF0000);
    pub const RANK_4: BitBoard = BitBoard(0xFF000000);
    pub const RANK_5: BitBoard = BitBoard(0xFF00000000);
    pub const RANK_6: BitBoard = BitBoard(0xFF0000000000);
    pub const RANK_7: BitBoard = BitBoard(0xFF000000000000);
    pub const RANK_8: BitBoard = BitBoard(0xFF00000000000000);

    // The files of the board.
    pub const FILE_A: BitBoard = BitBoard(0x0101010101010101);
    pub const FILE_B: BitBoard = BitBoard(0x0202020202020202);
    pub const FILE_C: BitBoard = BitBoard(0x0404040404040404);
    pub const FILE_D: BitBoard = BitBoard(0x0808080808080808);
    pub const FILE_E: BitBoard = BitBoard(0x1010101010101010);
    pub const FILE_F: BitBoard = BitBoard(0x2020202020202020);
    pub const FILE_G: BitBoard = BitBoard(0x4040404040404040);
    pub const FILE_H: BitBoard = BitBoard(0x8080808080808080);

    
    /// Creates a new BitBoard with all bits set to 0.
    pub fn new() -> Self {
        BitBoard(0)
    }

    /// Sets a bit at the given index.
    pub fn set_bit(mut self, index: u8) {
        self.0 |= 1 << index;
    }

    /// A function which returns an iterator over the squares of the BitBoard.
    pub fn iter_squares(mut self) -> impl Iterator<Item = Square> {
        (0..self.0.count_ones()).map(move |_| {
            //let non_zero = NonZeroU64::new_unchecked(self.0);
            let lsb = self.0.trailing_zeros() as u8;
            self = self.pop_lsb();
            Square::from_index(lsb)
        })
    }

    pub fn pop_lsb(self) -> BitBoard {
        self & (self - BitBoard(1)) 
    }


    /// Returns a BitBoard with one bit set at the given square (e.g. "A4").
    /// 
    ///  # Examples
    /// ```
    /// let A4 = BitBoard::from_square("A4");
    /// assert_eq!(A4.0, 0x0000000010000000);
    /// ```
    /// This BitBoard can then be OR'd with other BitBoards to represent a group of squares.
    pub fn from_square(square: &str) -> BitBoard {
        let letter = square.chars().nth(0).unwrap();
        let rank = square.chars().nth(1).unwrap().to_digit(10).unwrap();
        // Uses ASCII arithmetic. A = 0x41 in ASCII, B = 0x42, etc. so B - A = 1.
        let file_index = (letter as u8 - b'A') as u64;
        let rank_index = (rank - 1) as u64;
        let bit_index = rank_index * 8 + file_index;

        BitBoard(1 << bit_index)
    }

}

impl Deref for BitBoard {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl BitOr for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        BitBoard(self.0 | rhs.0)
    }
}

impl BitOr<u64> for BitBoard {
    type Output = Self;

    fn bitor(self, rhs: u64) -> Self::Output {
        BitBoard(self.0 | rhs)
    }
}

impl Sub for BitBoard {
    type Output = BitBoard;

    fn sub(self, rhs: BitBoard) -> Self::Output {
        BitBoard(self.0.wrapping_sub(rhs.0))
    }
}

impl BitAnd for BitBoard {
    type Output = BitBoard;

    fn bitand(self, rhs: BitBoard) -> Self::Output {
        BitBoard(self.0 & rhs.0)
    }
}


