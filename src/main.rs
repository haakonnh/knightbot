mod board;

use std::time::Instant;

use board::bitboard::BitBoard;
use board::board::Board;
use board::movegen::{get_attack_mask_for_bishop, get_attack_mask_for_queen, get_attack_mask_for_rook};
use board::square::Square;
use board::tables::{generate_rank_attack_table, precompute_masks};
use board::utils::print_bitboard;
fn main() {
    let board = Board::new();
    let rank_attacks = generate_rank_attack_table();
    let precomputed_masks = precompute_masks(); 

    let square = Square::A4; 
    let full_board = board.all_pieces;

    let start = Instant::now();
    let queen_attack_mask = get_attack_mask_for_queen(square, full_board, &rank_attacks, &precomputed_masks);
    let elapsed = start.elapsed();
    println!("Time elapsed for queen attack mask: {:?}", elapsed);
}
