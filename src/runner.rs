use std::env;
use std::fs;

use crate::registry;
use crate::utils::clear_terminal;

pub fn run() {
    clear_terminal();
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: aoc <day> [test|main]");
        std::process::exit(1);
    }

    let day: u32 = args[1].parse().expect("Day must be a number");
    let mode = args.get(2).map(String::as_str).unwrap_or("test");

    if mode != "test" && mode != "main" {
        eprintln!("Mode must be 'test' or 'main'");
        std::process::exit(1);
    }

    let input_path = format!("src/days/day{:02}/input_{}.txt", day, mode);

    let input = fs::read_to_string(&input_path).expect("Failed to read input file");

    let runner = registry::get(day).unwrap_or_else(|| panic!("Day {} not implemented", day));

    runner(&input);
}
