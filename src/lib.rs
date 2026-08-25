pub mod colors_out;
pub mod lights_out;

use std::collections::{HashMap, VecDeque};

pub const MOVE_MASKS: [(&str, u32); 25] = [
    ("A1", 0b1000000000000000000000000),
    ("B1", 0b0100000000000000000000000),
    ("C1", 0b0010000000000000000000000),
    ("D1", 0b0001000000000000000000000),
    ("E1", 0b0000100000000000000000000),
    ("A2", 0b0000010000000000000000000),
    ("B2", 0b0000001000000000000000000),
    ("C2", 0b0000000100000000000000000),
    ("D2", 0b0000000010000000000000000),
    ("E2", 0b0000000001000000000000000),
    ("A3", 0b0000000000100000000000000),
    ("B3", 0b0000000000010000000000000),
    ("C3", 0b0000000000001000000000000),
    ("D3", 0b0000000000000100000000000),
    ("E3", 0b0000000000000010000000000),
    ("A4", 0b0000000000000001000000000),
    ("B4", 0b0000000000000000100000000),
    ("C4", 0b0000000000000000010000000),
    ("D4", 0b0000000000000000001000000),
    ("E4", 0b0000000000000000000100000),
    ("A5", 0b0000000000000000000010000),
    ("B5", 0b0000000000000000000001000),
    ("C5", 0b0000000000000000000000100),
    ("D5", 0b0000000000000000000000010),
    ("E5", 0b0000000000000000000000001),
];

#[derive(Clone, Copy)]
pub enum Puzzle {
    LightsOut,
    ColorsOut,
}

impl Puzzle {
    pub fn start(self) -> u32 {
        match self {
            Self::LightsOut => lights_out::START,
            Self::ColorsOut => colors_out::START,
        }
    }

    pub fn moves(self) -> &'static [(&'static str, u32); 25] {
        match self {
            Self::LightsOut => &lights_out::MOVES,
            Self::ColorsOut => &colors_out::MOVES,
        }
    }

    fn check_victory(self, state: u32) -> i32 {
        match self {
            Self::LightsOut => lights_out::check_victory(state),
            Self::ColorsOut => colors_out::check_victory(state),
        }
    }
}

/// Find a shortest solution. The queue stores states only. The predecessor
/// state and move index are packed into one value because states use 25 bits.
pub fn solve_bfs_with_progress<F>(
    puzzle: Puzzle,
    start: u32,
    moves: &[(&'static str, u32); 25],
    mut progress: F,
) -> Option<Vec<&'static str>>
where
    F: FnMut(usize, usize),
{
    let mut queue = VecDeque::new();
    let mut predecessors: HashMap<u32, u32> = HashMap::new();
    queue.push_back(start);
    // The root points to itself and has no move (move index 31).
    predecessors.insert(start, (start << 5) | 31);
    let mut processed = 0;

    while let Some(state) = queue.pop_front() {
        if puzzle.check_victory(state) == 1 {
            let mut result = Vec::new();
            let mut current = state;
            while current != start {
                let encoded = predecessors[&current];
                result.push(moves[(encoded & 31) as usize].0);
                current = encoded >> 5;
            }
            result.reverse();
            return Some(result);
        }

        if puzzle.check_victory(state) == -1 {
            continue;
        }

        for (move_index, (_, move_mask)) in MOVE_MASKS.iter().enumerate() {
            if state & move_mask == 0 {
                continue;
            }

            let neighbor = state ^ moves[move_index].1;
            if let std::collections::hash_map::Entry::Vacant(entry) = predecessors.entry(neighbor) {
                entry.insert((state << 5) | move_index as u32);
                queue.push_back(neighbor);
            }
        }

        processed += 1;
        if processed % 100_000 == 0 {
            progress(processed, queue.len());
        }
    }

    None
}

pub fn solve(puzzle: Puzzle, start: u32) -> Option<Vec<&'static str>> {
    solve_bfs_with_progress(puzzle, start, puzzle.moves(), |_, _| {})
}

#[cfg(target_arch = "wasm32")]
fn parse_cell(cell: &str) -> Option<usize> {
    MOVE_MASKS.iter().position(|(name, _)| *name == cell)
}

#[cfg(target_arch = "wasm32")]
fn parse_move_configuration(configuration: &str) -> Result<[u32; 25], String> {
    let mut parsed = [0; 25];
    let rows: Vec<_> = configuration.lines().collect();
    if rows.len() != 25 {
        return Err("Move configuration must contain exactly 25 rows".to_string());
    }

    for (row_index, row) in rows.iter().enumerate() {
        let cells: Vec<_> = row.split(',').map(str::trim).collect();
        if cells.len() != 5 || cells.iter().any(|cell| cell.is_empty()) {
            return Err(format!(
                "Move {} must contain exactly five cells",
                MOVE_MASKS[row_index].0
            ));
        }

        let mut mask = 0;
        for cell in cells {
            let index = parse_cell(cell).ok_or_else(|| format!("Invalid cell name: {cell}"))?;
            let bit = MOVE_MASKS[index].1;
            if mask & bit != 0 {
                return Err(format!(
                    "Move {} contains a duplicate cell",
                    MOVE_MASKS[row_index].0
                ));
            }
            mask |= bit;
        }
        parsed[row_index] = mask;
    }
    Ok(parsed)
}

fn moves_to_configuration(moves: &[(&str, u32); 25]) -> String {
    moves
        .iter()
        .map(|(_, mask)| {
            MOVE_MASKS
                .iter()
                .filter_map(|(name, bit)| if mask & bit != 0 { Some(*name) } else { None })
                .collect::<Vec<_>>()
                .join(",")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn print_board(state: u32) {
    println!("   +---+---+---+---+---+");
    for row in 0..5 {
        print!(" {} |", row + 1);
        for column in 0..5 {
            let bit = (state >> (24 - row * 5 - column)) & 1;
            print!(" {} |", bit);
        }
        println!();
        println!("   +---+---+---+---+---+");
    }
    println!("     A   B   C   D   E");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_board_is_already_solved() {
        assert_eq!(solve(Puzzle::LightsOut, 0), Some(Vec::new()));
        assert_eq!(solve(Puzzle::ColorsOut, 0), Some(Vec::new()));
    }

    #[test]
    fn predecessor_path_reaches_empty_board() {
        let start = lights_out::MOVES[0].1;
        let moves = solve(Puzzle::LightsOut, start).expect("a solution should exist");
        let final_state = moves.iter().fold(start, |state, name| {
            let index = MOVE_MASKS
                .iter()
                .position(|(candidate, _)| candidate == name)
                .unwrap();
            state ^ lights_out::MOVES[index].1
        });
        assert_eq!(final_state, 0);
    }
}

#[cfg(target_arch = "wasm32")]
use js_sys::Function;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
pub type WasmError = JsValue;
#[cfg(not(target_arch = "wasm32"))]
pub type WasmError = String;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn solve_wasm(
    puzzle: &str,
    start: u32,
    configuration: &str,
    progress: &Function,
) -> Result<String, WasmError> {
    let puzzle = match puzzle {
        "lights" => Puzzle::LightsOut,
        "colors" => Puzzle::ColorsOut,
        _ => return Err(wasm_error("Unknown puzzle type")),
    };

    let masks = parse_move_configuration(configuration).map_err(|error| wasm_error(&error))?;
    let moves = std::array::from_fn(|index| (MOVE_MASKS[index].0, masks[index]));
    let progress = progress.clone();
    solve_bfs_with_progress(puzzle, start, &moves, move |processed, queue_size| {
        let _ = progress.call2(
            &JsValue::NULL,
            &JsValue::from_f64(processed as f64),
            &JsValue::from_f64(queue_size as f64),
        );
    })
    .map(|moves| moves.join(", "))
    .ok_or_else(|| wasm_error("No solution found"))
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen)]
pub fn default_moves_wasm(puzzle: &str) -> Result<String, WasmError> {
    let puzzle = match puzzle {
        "lights" => Puzzle::LightsOut,
        "colors" => Puzzle::ColorsOut,
        _ => return Err(wasm_error("Unknown puzzle type")),
    };
    Ok(moves_to_configuration(puzzle.moves()))
}

#[cfg(target_arch = "wasm32")]
fn wasm_error(message: &str) -> JsValue {
    JsValue::from_str(message)
}

#[cfg(not(target_arch = "wasm32"))]
fn wasm_error(message: &str) -> String {
    message.to_string()
}
