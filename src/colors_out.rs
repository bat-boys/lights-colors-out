// Board change masks for each move in Colors Out
//
// These must be tested in-game by pressing the cells and checking the resulting
// state. Changed cells are marked with stars.
//
// Each press changes exactly five cells: itself and four others.
pub const MOVES: [(&str, u32); 25] = [
    ("A1", 0b1001010000010000100000000),
    ("B1", 0b0101000000000010000100010),
    ("C1", 0b0010000010000000111000000),
    ("D1", 0b1001000010000110000000000),
    ("E1", 0b1000100100000100000000001),
    ("A2", 0b1000110000010000000000100),
    ("B2", 0b0101011000000010000000000),
    ("C2", 0b0000110100000000000010010),
    ("D2", 0b0000001010000000100011000),
    ("E2", 0b1100000101000000000100000),
    ("A3", 0b0011000000100100000100000),
    ("B3", 0b0100000000011001100000000),
    ("C3", 0b0000001001001010010000000),
    ("D3", 0b0000100100000100000000110),
    ("E3", 0b0010000001100011000000000),
    ("A4", 0b0010001001010001000000000),
    ("B4", 0b0000100001000000111000000),
    ("C4", 0b0000000000101001010000001),
    ("D4", 0b0000000110100000001001000),
    ("E4", 0b0000000000110000010101000),
    ("A5", 0b0000001000001100000010010),
    ("B5", 0b0100000010000000001101000),
    ("C5", 0b0010000000000000001000111),
    ("D5", 0b0010000000000000001000111),
    ("E5", 0b0000010000000001000010101),
];

pub fn check_victory(state: u32) -> i32 {
    // Check if the current state is a winning or losing state
    //
    // Winning state is when all board's cells are off (0)
    // Non-losing state is when all colors have at least one cell on (1)
    // Losing state is when at least one color has all cells off (0)
    //
    // Colors are like this in the board:
    //   +---+---+---+---+---+
    //   | R | R | G | C | C |
    //   +---+---+---+---+---+
    //   | R | R | G | C | C |
    //   +---+---+---+---+---+
    //   | G | G | G | G | G |
    //   +---+---+---+---+---+
    //   | M | M | G | Y | Y |
    //   +---+---+---+---+---+
    //   | M | M | G | Y | Y |
    //   +---+---+---+---+---+

    let red = state & 0b1100011000000000000000000;
    let cyan = state & 0b0001100011000000000000000;
    let magenta = state & 0b0000000000000001100011000;
    let yellow = state & 0b0000000000000000001100011;
    let gray = state & 0b0010000100111110010000100;

    if state == 0 {
        return 1; // win
    } else if red == 0 || cyan == 0 || magenta == 0 || yellow == 0 || gray == 0 {
        return -1; // lose
    } else {
        return 0;
    }
}
