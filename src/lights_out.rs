// Board change masks for each move in Lights Out
//
// These must be tested in-game by pressing the cells and checking the resulting
// state. Changed cells are marked with stars.
//
// Each press changes exactly five cells: itself and four others.
pub const MOVES: [(&str, u32); 25] = [
    ("A1", 0b1000000100000010001000001),
    ("B1", 0b0101000000000001001100000),
    ("C1", 0b0011000000001000001001000),
    ("D1", 0b0011000000010000000000110),
    ("E1", 0b1100100010000000000000100),
    ("A2", 0b0100010000100010000010000),
    ("B2", 0b0000001001010000010000100),
    ("C2", 0b0010000100100100000000010),
    ("D2", 0b0000001010000010100001000),
    ("E2", 0b0001010001000000010000001),
    ("A3", 0b0000100010110000000010000),
    ("B3", 0b0100000000110000100001000),
    ("C3", 0b0010110000001000000000001),
    ("D3", 0b1000100100000100000000010),
    ("E3", 0b0000010000001010000110000),
    ("A4", 0b0010001000000001010010000),
    ("B4", 0b0000001100100000100000001),
    ("C4", 0b0000000001000101010000010),
    ("D4", 0b0001010000000001001001000),
    ("E4", 0b1000000000000100010100100),
    ("A5", 0b1000100001000000000110000),
    ("B5", 0b0100000001000000101001000),
    ("C5", 0b0000000100001010100000100),
    ("D5", 0b0000001010010100000000010),
    ("E5", 0b0000000010001001000100001),
];

pub fn check_victory(state: u32) -> i32 {
    // Check if the current state is a winning or losing state
    //
    // Winning state is when all board's cells are off (0)
    // Non-losing state is when all colors have at least one cell on (1)

    if state == 0 {
        return 1;
    } else {
        return 0;
    }
}
