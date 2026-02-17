use std::vec;

pub fn run(input: &str) {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    let mut total_part1: u32 = 0;
    let mut total_part2: u32 = 0;
    let groups = get_groups(lines.clone());
    // println!("groups: {:?}", groups);
    for group in groups {
        let common_char: char = get_common_char(group[0], group[1], group[2]);
        let char_number = get_char_number(&common_char);
        total_part2 += char_number;
    }
    println!("part2: {}", total_part2);

    // println!("{:?}", lines);
    for line in lines {
        let splits: Vec<&str> = split_items(line);
        // println!("{:?}", splits);
        let common_char: char = check_items(splits[0], splits[1]);
        // println!("common_char: {}", common_char);
        let char_number = get_char_number(&common_char);
        // println!("char_number: {}", char_number);
        total_part1 += char_number;
    }

    println!("part1: {}", total_part1);
}

fn split_items(input_item: &str) -> Vec<&str> {
    let half = input_item.len() / 2;
    vec![&input_item[..half], &input_item[half..]]
}

fn check_items(left: &str, right: &str) -> char {
    left.chars()
        .find(|c| right.contains(*c))
        .unwrap_or_else(|| panic!("No match between left: {} and right: {}", left, right))
}

fn get_char_number(input_char: &char) -> u32 {
    if input_char.is_ascii_lowercase() {
        (*input_char as u32) - ('a' as u32) + 1
    } else if input_char.is_ascii_uppercase() {
        (*input_char as u32) - ('A' as u32) + 27
    } else {
        panic!("Invalid character: {}", input_char)
    }
}

fn get_groups(input_lines: Vec<&str>) -> Vec<Vec<&str>> {
    input_lines.chunks(3).map(|c| c.to_vec()).collect()
}

fn get_common_char(sack_1: &str, sack_2: &str, sack_3: &str) -> char {
    sack_1
        .chars()
        .find(|c| sack_2.contains(*c) && sack_3.contains(*c))
        .expect("No common chars found")
}
