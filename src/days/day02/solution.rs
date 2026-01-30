
pub fn run(input: &str) {
    let lines: Vec<&str> = input
        .lines()
        .filter(|l| !l.is_empty())
        .collect();

    println!("{:?}", lines);
}
