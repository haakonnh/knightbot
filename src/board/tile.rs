use super::piece::{Piece, Color};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tile {
    Empty,
    Occupied {piece: Piece, color: Color},
}