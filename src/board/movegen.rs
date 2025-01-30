use core::time;
use std::time::Instant;

use super::{bitboard::{self, BitBoard}, board::{self, Board}, piece::Piece, square::{self, Square}, tables::{PrecomputedMasks, SlidingAttackTable}, utils::{pdep, pext, print_bitboard}};



pub fn fetch_psuedo_legal_moves(piece: Piece, square: Square, occupation: BitBoard) -> BitBoard {
    BitBoard(1)
}

/// This function generates the attack mask for a bishop on a given square.
/// It requires the precomputed rank_attacks structure, which contains attack masks for 
/// a given square and rank occupancy (this is directly used for rank attacks for rooks).
/// This function uses this structure to generate the attack mask for a bishop by applying pext with
/// the precomputed diagonal masks for the given square and the entire board occupancy bitboard, thereby essentially
/// treating the diagonal as a rank. However, since diagonals can be of different lengths than 8, the square index
/// needs to be offset correctly.
///
/// # Arguments
///
/// * `square` - The square on which the bishop is located.
/// * `occupancy` - The bitboard representing the occupancy of the entire board.
/// * `rank_attacks` - The precomputed rank attack table.
/// * `precomputed_masks` - The precomputed masks for bishops and rooks.
///
/// # Returns
///
/// * `BitBoard` - The attack mask for the bishop on the given square.
///
/// # Performance
///
/// This function takes about 3 microseconds to generate the attack mask for a bishop on a given square.
///
/// # Example
///
/// ```
/// let board = Board::new();
/// let rank_attacks = generate_rank_attack_table();
/// let precomputed_masks = precompute_masks();
/// let square = Square::B6;
/// let full_board = board.all_pieces;
/// let attack_mask = get_attack_mask_for_bishop(square, full_board, rank_attacks, precomputed_masks);
/// print_bitboard(attack_mask);
/// ```
pub fn get_attack_mask_for_bishop(square: Square, occupancy: BitBoard, rank_attacks: &SlidingAttackTable, precomputed_masks: &PrecomputedMasks) -> BitBoard {
    let mut total_mask = 0u64;
    let bishop_masks = precomputed_masks.bishop_masks.get(&square).unwrap();
    for mask in bishop_masks {
        let extracted = pext(*occupancy, *mask);
        let first_one = mask.trailing_zeros() as u8;
        let first_square_rank = Square::from_index(first_one).rank();
        let attack_mask = *rank_attacks.get(square.rank() - first_square_rank, extracted).unwrap();
        let deposited = pdep(attack_mask, *mask);
        total_mask |= deposited;
    }
    BitBoard(total_mask)
}

pub fn get_attack_mask_for_rook(square: Square, occupancy: BitBoard, rank_attacks: &SlidingAttackTable, precomputed_masks: &PrecomputedMasks) -> BitBoard {
    let mut total_mask = 0u64;
    let rook_masks = precomputed_masks.rook_masks.get(&square).unwrap();
    let mut i = 0 as u8;
    let mut index = 0;
    for mask in rook_masks {
        if i == 0 {
            index = square.rank();
        }
        else {
            index = square.file();
        }
        let extracted = pext(*occupancy, *mask);
        let attack_mask = *rank_attacks.get(index, extracted).unwrap();
        let deposited = pdep(attack_mask, *mask);
        total_mask |= deposited;
        i += 1;
    }
    BitBoard(total_mask)
}

pub fn get_attack_mask_for_queen(square: Square, occupancy: BitBoard, rank_attacks: &SlidingAttackTable, precomputed_masks: &PrecomputedMasks) -> BitBoard {
    let diagonal_mask = get_attack_mask_for_bishop(square, occupancy, rank_attacks, precomputed_masks);
    let rook_mask = get_attack_mask_for_rook(square, occupancy, rank_attacks, precomputed_masks);

    BitBoard(diagonal_mask.0 | rook_mask.0)
}