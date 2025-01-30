use core::arch::x86_64::{_pext_u64, _pdep_u64};
use core::mem::size_of;
use std::collections::HashMap;


use super::tables::SlidingAttackTable;

/// A function to extract bits from a source using a mask.
pub fn pext(source: u64, mask: u64) -> u64 {
    unsafe { _pext_u64(source, mask) }
}

pub fn pdep(source: u64, mask: u64) -> u64 {
    unsafe { _pdep_u64(source, mask) }
}

/// A function to print a bitboard to the console.
pub fn print_bitboard(bitboard: u64) {
    const LAST_BIT: u64 = 63;
    for rank in 0..8 {
        print!("{} ", 8 - rank);
        for file in (0..8).rev() {
            let mask = 1u64 << (LAST_BIT - (rank * 8) - file);
            let char = if bitboard & mask != 0 { '1' } else { '0' };
            print!("{char} ");
        }
        println!();
    }
    println!("  A B C D E F G H");
}

/// A function to check the memory usage of the SlidingAttackTable struct.
pub fn check_memory_of_table(attacks: SlidingAttackTable) {
        // Size of the RankAttack struct itself
        let size_of_rank_attack = size_of::<SlidingAttackTable>();
        println!("Size of RankAttack struct: {} bytes", size_of_rank_attack);
    
        // Calculate the total memory usage
        let mut total_size = size_of_rank_attack;
        for hashmap in &attacks.table {
            total_size += size_of::<HashMap<u64, u64>>();
            for (key, value) in hashmap {
                total_size += size_of::<u64>() * 2; // Size of key and value
            }
        }
        println!("Approximate total memory usage: {} bytes", total_size);
}
