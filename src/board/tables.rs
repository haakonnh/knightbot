use std::collections::HashMap;
use super::{bitboard::BitBoard, square::Square, utils::{pdep, pext}};


pub struct AttackTables {
    pub rook_attacks: SlidingAttackTable,
}


#[derive(Debug, Clone)]
pub struct SlidingAttackTable {
    pub table: Vec<HashMap<u64, u64>>,
}

pub struct BishopAttackTable {
    pub table: HashMap<Square, u64>,
}

impl BishopAttackTable {
    pub fn new() -> Self {
        BishopAttackTable {
            table: HashMap::new(),
        }
    }

    pub fn insert(&mut self, square: Square, mask: u64, value: u64) {
        self.table.insert(square, value);
    }

    pub fn get(&self, square: Square, mask: u64) -> Option<&u64> {
        self.table.get(&square)
    }
}


pub struct KnightAttackTable {
    pub table: HashMap<Square, u64>,
}


impl SlidingAttackTable {
    pub fn new() -> Self {
        SlidingAttackTable { 
            table: vec![HashMap::new(); 64],
         }
    }

    pub fn insert(&mut self, index: u8, mask: u64, value: u64) {
        self.table[index as usize].insert(mask, value);
    }

    pub fn get(&self, index: u8, mask: u64) -> Option<&u64> {
        self.table[index as usize].get(&mask)
    }
}

pub fn test_attack_mask() {
    let file = 4;
    let mask = 0b11110100;
    let mut attacks = 0;
    println!("Mask {:b}", mask);
    for i in (0..file).rev() {
        attacks |= 1 << i;
        if (mask & (1 << i)) != 0 {
            break;
        }
    }

    for i in (file+1)..8 {
        attacks |= 1 << i;
        if (mask & (1 << i)) != 0 {
            break;
        }
    }
    println!("{:b}", attacks);
}

/// Generates a table of all possible rank attacks for each square and rank state mask. 
/// NOTE: This function is only called once at the start of the program to generate the table.
/// TODO: This table does not really need to include the rank number, as the attack mask is the same for all ranks. It is also possible to neglect edge squares.
/// 
pub fn generate_rank_attack_table() -> SlidingAttackTable {
    let mut rank_attacks = SlidingAttackTable::new();
    let full = BitBoard::FULL;
    // Loop over all possible squares.
    for square in full.iter_squares() {
        let file = square.file() ;
        // Generate all possible permutations of pieces on the same rank.
        for mask in 0..256 as u64 {
            let mut attacks = 0;

            // Calculate the attack mask for the current permutation.
            // Go towards the least significant bit (towards the A file).
            for i in (0..file ).rev() {
                attacks |= 1 << i;
                if (mask & (1 << i)) != 0 {
                    break;
                }
            }

            // Go towards the most significant bit (towards the H file).
            for i in (file + 1)..8 {
                attacks |= 1 << i;
                if (mask & (1 << i)) != 0 {
                    break;
                }
            }

            rank_attacks.insert(square.file(), mask as u64, attacks);
        }

    }
    rank_attacks
}


pub struct PrecomputedMasks {
    pub rook_masks: HashMap<Square, Vec<u64>>,
    pub bishop_masks: HashMap<Square, Vec<u64>>,
}


pub fn precompute_masks() -> PrecomputedMasks {
    let mut rook_masks = HashMap::new();
    let mut bishop_masks = HashMap::new();

    for square in BitBoard::FULL.iter_squares() {
        let  file = square.file();
        let rank = square.rank();

        // Rook masks
        let mut rook_mask_for_square = Vec::new();
        let mut file_mask = 0u64;
        let mut rank_mask = 0u64;
        for i in 0..8 {
            file_mask |= 1 << (i * 8 + file);
            rank_mask |= 1 << (rank * 8 + i);
        }
        rook_mask_for_square.push(file_mask);
        rook_mask_for_square.push(rank_mask);
        rook_masks.insert(square, rook_mask_for_square);

        // Bishop masks (diagonals)
        let mut bishop_masks_for_square = Vec::new();
        let mut diag1_mask = 0u64;
        let mut diag2_mask = 0u64;

        for i in 0..8 {
            let mut diag1_file = file as i8 + i;
            let mut diag1_rank = rank as i8 + i;
            if diag1_file < 8 && diag1_rank < 8 {
                diag1_mask |= 1 << (diag1_rank * 8 + diag1_file);
            }

            diag1_file = file as i8 - i;
            diag1_rank = rank as i8 - i;
            if diag1_file >= 0 && diag1_rank >= 0 {
                diag1_mask |= 1 << (diag1_rank * 8 + diag1_file);
            }

            let mut diag2_file = file as i8 + i;
            let mut diag2_rank = rank as i8 - i;
            if diag2_file < 8 && diag2_rank >= 0 {
                diag2_mask |= 1 << (diag2_rank * 8 + diag2_file);
            }

            diag2_file = file as i8 - i;
            diag2_rank = rank as i8 + i;
            if diag2_file >= 0 && diag2_rank < 8 {
                diag2_mask |= 1 << (diag2_rank * 8 + diag2_file);
            }




        }

        bishop_masks_for_square.push(diag1_mask);
        bishop_masks_for_square.push(diag2_mask);
        bishop_masks.insert(square, bishop_masks_for_square);
    

    }

    PrecomputedMasks {
        rook_masks,
        bishop_masks,
    }

}

/* pub fn precompute_bishop_attack_masks(rank_attacks: &SlidingAttackTable, precomputed_masks: PrecomputedMasks) -> BishopAttackTable {
    let mut bishop_attacks = BishopAttackTable::new();
    for square in BitBoard::FULL.iter_squares() {
        let piece_masks = precomputed_masks.bishop_masks.get(&square).unwrap();
        
        for occupancy in 0..256 as u64 {
            let extracted = pext(occupancy, *mask);
    
            let first_one = mask.trailing_zeros() as u8;
            let first_square_rank = Square::from_index(first_one).rank();
        
            let attack_mask = *rank_attacks.get(square.rank() - first_square_rank, extracted).unwrap();
        
            
            let deposited = pdep(attack_mask, *mask);
                
            bishop_attacks.insert(square, occupancy, deposited);
            }

            
            
            
        }

        
    

    bishop_attacks
} */