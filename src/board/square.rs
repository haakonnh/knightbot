use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Square {
    A1 = 0, B1 = 1, C1 = 2, D1 = 3, E1 = 4, F1 = 5, G1 = 6, H1 = 7,
    A2 = 8, B2 = 9, C2 = 10, D2 = 11, E2 = 12, F2 = 13, G2 = 14, H2 = 15,
    A3 = 16, B3 = 17, C3 = 18, D3 = 19, E3 = 20, F3 = 21, G3 = 22, H3 = 23,
    A4 = 24, B4 = 25, C4 = 26, D4 = 27, E4 = 28, F4 = 29, G4 = 30, H4 = 31,
    A5 = 32, B5 = 33, C5 = 34, D5 = 35, E5 = 36, F5 = 37, G5 = 38, H5 = 39,
    A6 = 40, B6 = 41, C6 = 42, D6 = 43, E6 = 44, F6 = 45, G6 = 46, H6 = 47,
    A7 = 48, B7 = 49, C7 = 50, D7 = 51, E7 = 52, F7 = 53, G7 = 54, H7 = 55,
    A8 = 56, B8 = 57, C8 = 58, D8 = 59, E8 = 60, F8 = 61, G8 = 62, H8 = 63,
}

impl Square {
    /// Returns a square from an index of a u64.
    pub fn from_index(index: u8) -> Square {
        match Square::try_from(index) {
            Ok(square) => square,
            Err(_) => panic!("Index out of bounds for Square"),
        }
    }

    /// Return the file of the square. 0x7 is a mask to get the last 3 bits.
    pub fn file(self) -> u8 {
        (self as u8) & 0x7
    }

    /// Return the rank of the square. Bitwise shift right 3 to get the rank.
    pub fn rank(self) -> u8 {
        (self as u8).wrapping_shr(3)
    }
}

impl fmt::Display for Square {
    /// Gives the square's pure algebraic coordinates notation.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const FILES: [char; 8] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h'];
        const RANKS: [char; 8] = ['1', '2', '3', '4', '5', '6', '7', '8'];

        write!(f, "{}{}", FILES[self.file() as usize], RANKS[self.rank() as usize])
    }
}

impl TryFrom<u8> for Square {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < 64 {
            Ok(unsafe { std::mem::transmute(value) })
        } else {
            Err("Value out of bounds for Square")
        }
    }
}