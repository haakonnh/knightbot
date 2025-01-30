use super::bitboard::BitBoard;

pub struct Board {
    //pub tiles: [[Tile; 8]; 8]
    pub white_pieces: BitBoard,
    pub black_pieces: BitBoard,
    pub white_pawns: BitBoard,
    pub black_pawns: BitBoard,
    pub white_knights: BitBoard,
    pub black_knights: BitBoard,
    pub white_bishops: BitBoard,
    pub black_bishops: BitBoard,
    pub white_rooks: BitBoard,
    pub black_rooks: BitBoard,
    pub white_queens: BitBoard,
    pub black_queens: BitBoard,
    pub white_king: BitBoard,
    pub black_king: BitBoard,
    pub all_pieces: BitBoard,
}

impl Board {
    pub fn new() -> Self {
        Self {
            white_pieces: BitBoard(0x000000000000FFFF),
            black_pieces: BitBoard(0xFFFF000000000000),

            all_pieces: BitBoard(0xFFFF00000000FFFF),

            white_pawns: BitBoard(0x000000000000FF00),
            black_pawns: BitBoard(0x00FF000000000000),
            white_knights: BitBoard(0x0000000000000042),
            black_knights: BitBoard(0x4200000000000000),
            white_bishops: BitBoard(0x0000000000000024),
            black_bishops: BitBoard(0x2400000000000000),
            white_rooks: BitBoard(0x0000000000000081),
            black_rooks: BitBoard(0x8100000000000000),
            white_queens: BitBoard(0x0000000000000008),
            black_queens: BitBoard(0x0800000000000000),
            white_king: BitBoard(0x0000000000000010),
            black_king: BitBoard(0x1000000000000000),
            
        }
    }

    // updates the different bitboards containing groupings of bitboards
    pub fn update(&mut self) {
        self.white_pieces = self.white_pawns | self.white_knights | self.white_bishops | self.white_rooks | self.white_queens | self.white_king;
        self.black_pieces = self.black_pawns | self.black_knights | self.black_bishops | self.black_rooks | self.black_queens | self.black_king;
        self.all_pieces = self.white_pieces | self.black_pieces;

    }
    
        

}
