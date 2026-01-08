// Board change masks for each move in Lights Out
//
// These must be tested in-game by pressing the cells and checking the resulting
// state. Changed cells are marked with stars.
//
// Each press changes exactly five cells: itself and four others.

pub const START: u32 = 0b00100_01001_10000_00001_00111;
pub const MOVES: [(&str, u32); 25] = [
    ("A1", 0b10000_00000_00010_01010_01000),
    ("B1", 0b01011_00000_00101_00000_00000),
    ("C1", 0b00110_00000_00101_00000_00100),
    ("D1", 0b00010_01001_00000_01000_10000),
    ("E1", 0b00001_00001_01000_00010_00010),
    ("A2", 0b00000_10101_00000_00001_00001),
    ("B2", 0b01000_01000_10001_10000_00000),
    ("C2", 0b00100_00110_01001_00000_00000),
    ("D2", 0b11000_00010_10000_01000_00000),
    ("E2", 0b00001_00111_00000_00001_00000),
    ("A3", 0b00100_00001_10000_00000_10001),
    ("B3", 0b00000_00110_11000_00010_00000),
    ("C3", 0b10001_00000_00100_10000_00001),
    ("D3", 0b01000_01000_00010_00101_00000),
    ("E3", 0b00000_00100_01001_00010_00100),
    ("A4", 0b00010_00000_01000_11000_10000),
    ("B4", 0b10000_10000_00000_01100_00100),
    ("C4", 0b00010_10000_00000_10101_00000),
    ("D4", 0b00000_11000_00000_00010_10100),
    ("E4", 0b10000_01000_00000_00001_01010),
    ("A5", 0b00100_00000_00100_00100_11000),
    ("B5", 0b00000_10000_00110_00000_01010),
    ("C5", 0b00000_00000_10010_00000_01101),
    ("D5", 0b01000_00010_00000_10100_00010),
    ("E5", 0b00101_00000_00010_00000_00011),
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
