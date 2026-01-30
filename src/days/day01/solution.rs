use std::vec;

pub fn run(input: &str) {
    let mut lines: Vec<&str> = input.lines().collect();
    lines.push("");

    // println!("{:?}", lines);

    let cleaned_lines = clean_lines(lines);
    // println!("{:?}", cleaned_lines);

    let max_food = get_max(cleaned_lines.clone());
    println!("part1: {}", max_food);

    let sorted_totals: Vec<i32> = sort_totals(cleaned_lines);
    println!("part2: {:?}", sorted_totals.iter().sum::<i32>());
}

fn sort_totals(cleaned_lines: Vec<Vec<i32>>) -> Vec<i32> {
    let mut totals: Vec<i32> = vec![];
    for item in cleaned_lines {
        totals.push(item.iter().sum());
    }
    totals.sort();
    totals.reverse();

    totals[0..3].to_vec()
}

fn get_max(cleaned_lines: Vec<Vec<i32>>) -> i32 {
    let mut max_food: i32 = 0;
    for item in cleaned_lines {
        let current_sum = item.iter().sum();
        if current_sum > max_food {
            max_food = current_sum;
        }
    }
    max_food
}

fn clean_lines(lines: Vec<&str>) -> Vec<Vec<i32>> {
    let mut elf_food: Vec<Vec<i32>> = vec![];
    let mut temp: Vec<i32> = vec![];

    for i in 0..(lines.len() - 1) {
        let current = lines[i];
        let next = lines[i + 1];
        // println!("current: {}, next: {}", current, next);
        if !current.is_empty() {
            match next {
                "" => {
                    temp.push(
                        current
                            .parse()
                            .unwrap_or_else(|_| panic!("Failed to parse: {}", current)),
                    );
                    elf_food.push(temp.clone());
                    temp.clear();
                }
                _ => temp.push(
                    current
                        .parse()
                        .unwrap_or_else(|_| panic!("Failed to parse: {}", current)),
                ),
            }
        }
    }
    elf_food
}
