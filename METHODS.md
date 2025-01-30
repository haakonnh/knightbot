bit-parallel operations:
E.g. easily check if a pawn is a passed pawn by making a bitmap that contains 1's on squares that if all these were empty of enemy pawns, would make the pawn passed. Then, AND this map with the map of enemy pawns. If the resulting map is empty, the pawn is passed.

occupied_squares^=set_mask[from]|set_mask[to]:
The from square should be occupied as it contains the piece to be moved. The to square should be empty as this is a non-capture, so the or of these two would total two masks on the board. This is XOR'ed to the board as this clears the from and initiates the to.

Using the already described method of generating the attack bitmaps (recall that generating attack bitmaps for non-sliding pieces is trivial since all the squares a given type of piece attacks can be computed at program startup since this doesn't change like those of sliding pieces) we compute the bishop attacks for E4, then AND this with the bitmap of white/black bishops and queens, which gives us 1 bits for each bishop/queen that attacks this target square, regardless of the color. We save this result, and then repeat this for the rook moves and rooks/queens bitmap. Then we take the knight attack bitmap, AND this with the white/black knight bitmap, and repeat for the king and pawns. We end up with five bitmaps that when ORed together, enumerate every square that is attacking E4.

PEXT makes it easy to extract blocker pieces on diagonals and files, as it doesnt care if the bitstring is contiguous. 

We have a precomputed table of e.g. bishop moves for any arbitrary square, e.g. D4, for all possible permutation of blocker pieces. These values will be compressed by PEXT. In the engine, firstly, the occupied bitboard will be PEXT'ed with the masks that contain the relevant ranks, files or diagonals. Then, we will index the precomputed table with the square of the piece and the PEXT value which contains the state of the rank, file or diagonal.


SIMPLY USE ONE SLIDING PIECE ATTACK TABLE, AND USE PEXT TO EXTRACT THE RELEVANT RAY AND LOOK IT UP IN THE ATTACK TABLE. CREATE PREDEFINED MASKS FOR THE RELEVANT RAY FOR EACH SQUARE.



BIG ISSUE:
Sliding attack table takes SQUARE as input. For diagonals, this is problematic as the diagonals the bishop is one is converted to a rank with PEXT. The sliding attack table therefore needs to take some other argument instead, representing where in the line it sits.

SO.
All diagonal-attack masks need to be adjusted to the length of the diagonal. 
To solve this, count the ones in the initial precomputed diagonal mask, and extract
the X most significant bits from the attack mask.

Diagonal and anti-diagonal attack masks need to be handled differently. 
Anti-diagonal [1] must have the 8bit number attack mask reversed after retrieval as the bits end up mirrored. 

Handle some error for A8, as the diagonal is still initialized to 1 on the square where it stands even though it stands on this square.
