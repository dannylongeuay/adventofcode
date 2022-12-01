/*
 * This file contains template code.
 * There is no need to edit this file unless you want to change template functionality.
 */
use advent_of_code::{run_solution, ANSI_BOLD, ANSI_ITALIC, ANSI_RESET};
use std::process;

fn main() {
    let args = match advent_of_code::parse_args() {
        Ok(day) => day,
        Err(e) => {
            eprintln!("Failed to process arguments: {}", e);
            process::exit(1);
        }
    };

    let total = run_solution(args.year, args.day);

    println!(
        "{}Total:{} {}{:.2}ms{}",
        ANSI_BOLD, ANSI_RESET, ANSI_ITALIC, total, ANSI_RESET
    );
}
