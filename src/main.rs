use std::env;

use lights_colors_out::{print_board, solve_bfs_with_progress, Puzzle};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} --lights-out | --colors-out", args[0]);
        std::process::exit(1);
    }

    let puzzle = match args[1].as_str() {
        "--lights-out" => Puzzle::LightsOut,
        "--colors-out" => Puzzle::ColorsOut,
        _ => {
            eprintln!("Invalid argument. Use --lights-out or --colors-out.");
            std::process::exit(1);
        }
    };

    let start = puzzle.start();
    println!("Starting state:");
    print_board(start);
    println!();

    match solve_bfs_with_progress(puzzle, start, puzzle.moves(), |processed, queue_size| {
        if processed % 100_000 == 0 {
            println!("Processed {} moves, queue size: {}", processed, queue_size);
        }
    }) {
        Some(moves) => println!("Moves: {}", moves.join(", ")),
        None => println!("No solution found."),
    }
}
