mod colors_out;
mod lights_out;

use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::env;

/*
 * Solvers for BatMUD gala 2025 Burglefloogah quest parts Lights Out and Colors Out
 *
 * Boards are 5x5 grids, represented as 25-bit integers so that A1 is the most
 * significant bit.
 */

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

#[derive(Clone)]
pub struct Node {
    // Represents a node in the search tree for the burgle board games.
    pub state: u32,
    pub parent: Option<Box<Node>>, // Use Box to allow recursive types
    pub move_name: Option<String>,
}

pub fn print_moves(node: Node) {
    // Prints the moves taken to reach the current node via parent nodes.
    // The moves are printed in the order they were made, starting from the root node.

    let mut moves: Vec<Option<String>> = vec![node.move_name.clone()];
    let mut current = &node.parent;
    while let Some(previous) = current {
        moves.push(previous.move_name.clone());
        current = &previous.parent;
    }
    moves.reverse();

    println!(
        "Moves: {}",
        moves
            .iter()
            .filter_map(|m| m.as_ref())
            .map(|m| m.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

pub fn print_board(state: u32) {
    // Prints the current state of the board in a 5x5 grid format.

    println!("   +---+---+---+---+---+");
    // get all bits from state to a vec
    let bits: Vec<u32> = (0..25)
        .map(|i| (state >> i) & 1)
        .rev()
        .collect::<Vec<u32>>();

    for i in 0..5 {
        print!(" {} |", i + 1);
        for j in 0..5 {
            let bit = bits[i * 5 + j];
            print!(" {} |", if bit == 1 { "1" } else { "0" });
        }
        println!();
        println!("   +---+---+---+---+---+");
    }
    println!("     A   B   C   D   E");
}

pub fn solve_bfs(start: Node, moves: [(&str, u32); 25], check_victory: fn(u32) -> i32) {
    // Solve the board game using breadth-first search (BFS).

    let board_moves: HashMap<&str, u32> = moves.iter().cloned().collect();
    let board_move_masks: HashMap<&str, u32> = MOVE_MASKS.iter().cloned().collect();

    let mut visited: HashSet<u32> = HashSet::new();
    let mut queue: VecDeque<Node> = VecDeque::new();

    queue.push_back(start);
    let mut i = 0;

    while let Some(node) = queue.pop_front() {
        visited.insert(node.state);
        let victory = check_victory(node.state);

        if victory == 1 {
            print_moves(node);
            break;
        } else if victory == -1 {
            // Losing condition, skip this branch
            continue;
        }

        // otherwise continue exploring, add all possible moves
        // that haven't been visited yet to queue

        for (move_name, move_mask) in board_move_masks.iter() {
            if node.state & move_mask != 0 {
                let neighbor_state = node.state ^ board_moves[move_name];
                if !visited.contains(&neighbor_state) {
                    queue.push_back(Node {
                        state: neighbor_state,
                        parent: Some(Box::new(node.clone())),
                        move_name: Some(move_name.to_string()),
                    });
                }
            }
        }

        i += 1;
        if i % 100000 == 0 {
            println!("Processed {} moves, queue size: {}", i, queue.len());
        }
    }
}

fn main() {
    // Read starting state from command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!(
            "Usage: {} --lights-out | --colors-out <starting state>",
            args[0]
        );
        std::process::exit(1);
    }

    let (moves, check_victory): ([(&str, u32); 25], fn(u32) -> i32) = match args[1].as_str() {
        "--lights-out" => (lights_out::MOVES, lights_out::check_victory),
        "--colors-out" => (colors_out::MOVES, colors_out::check_victory),
        _ => {
            eprintln!("Invalid argument. Use --lights-out or --colors-out.");
            std::process::exit(1);
        }
    };

    let start_state = u32::from_str_radix(&args[2], 2)
        .expect("Invalid starting state. Must be a 25-bit integer.");

    let start = Node {
        state: start_state,
        parent: None,
        move_name: None,
    };
    println!("Starting state:");
    print_board(start.state);
    println!("");
    solve_bfs(start, moves, check_victory);
}
