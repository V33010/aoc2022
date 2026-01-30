use std::fs;
use std::path::Path;

fn main() {
    let day: u32 = std::env::args()
        .nth(1)
        .expect("Usage: generator <day>")
        .parse()
        .expect("Day must be a number");

    if !(1..=25).contains(&day) {
        panic!("Day must be between 1 and 25");
    }

    let day_str = format!("day{:02}", day);
    let day_dir = format!("src/days/{}", day_str);

    fs::create_dir_all(&day_dir).unwrap();

    create_if_missing(&format!("{}/solution.rs", day_dir), SOLUTION_TEMPLATE);
    create_if_missing(&format!("{}/mod.rs", day_dir), DAY_MOD_TEMPLATE);
    create_if_missing(&format!("{}/input_test.txt", day_dir), "");
    create_if_missing(&format!("{}/input_main.txt", day_dir), "");

    regenerate_days_mod();
    regenerate_registry();

    println!("✅ Generated {}", day_str);
}

fn create_if_missing(path: &str, contents: &str) {
    if !Path::new(path).exists() {
        fs::write(path, contents).unwrap();
    }
}

fn regenerate_days_mod() {
    let mut mods = String::new();

    for entry in fs::read_dir("src/days").unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if name.starts_with("day") {
            mods.push_str(&format!("pub mod {};\n", name));
        }
    }

    fs::write("src/days/mod.rs", mods).unwrap();
}

fn regenerate_registry() {
    let mut body = String::new();

    body.push_str("use crate::days::*;\n\n");
    body.push_str("pub fn get(day: u32) -> Option<fn(&str)> {\n");
    body.push_str("    match day {\n");

    for entry in fs::read_dir("src/days").unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();
        if name.starts_with("day") {
            let num: u32 = name[3..].parse().unwrap();
            body.push_str(&format!("        {} => Some({}::run),\n", num, name));
        }
    }

    body.push_str("        _ => None,\n");
    body.push_str("    }\n}\n");

    fs::write("src/registry.rs", body).unwrap();
}

const DAY_MOD_TEMPLATE: &str = r#"
pub mod solution;
pub use solution::run;
"#;

const SOLUTION_TEMPLATE: &str = r#"
pub fn run(input: &str) {
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    println!("{:?}", lines);
}
"#;
