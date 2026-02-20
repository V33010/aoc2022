pub fn run(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    // println!("{:?}", lines);
    let (stacks, steps) = parse_input(lines);
    print_stacks(stacks.clone());
    println!("");
    // print_steps(steps.clone());
    println!("");
    let parsed_steps = parse_steps(steps);
    // print_parsed_steps(parsed_steps);
    let gathered_stacks = gather_stacks(stacks);
    print_gathered_stacks(gathered_stacks.clone());
    let inverted_stacks = invert_matrix(gathered_stacks);

    for line in inverted_stacks {
        println!("{:?}", line);
    }
}

fn parse_input(lines: Vec<&str>) -> (Vec<&str>, Vec<&str>) {
    let blank_loc = lines
        .iter()
        .position(|&s| s.is_empty())
        .unwrap_or_else(|| panic!("blank element not found in input"));
    // println!("{}", blank_loc);
    let next: usize = blank_loc + 1;
    let prev: usize = blank_loc - 1;
    (lines[0..prev].to_vec(), lines[next..].to_vec())
}

fn invert_matrix(input_matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let height = input_matrix.len();
    println!("height: {}", height);
    let width = input_matrix[0].len();
    println!("width: {}", width);
    let mut output: Vec<Vec<char>> = vec![vec![]];
    for i in 0..height {
        let mut temp: Vec<char> = vec![];
        for j in 0..width {
            temp.push(input_matrix[j][i])
        }
        output.push(temp);
    }
    output
}

fn gather_stacks(stacks: Vec<&str>) -> Vec<Vec<char>> {
    let width = stacks[0].len() as i32;
    let total_horizontal = (width + 1) / 3;
    println!("total_horizontal: {}", total_horizontal);
    println!("width: {}", width);
    let mut intermediate: Vec<Vec<char>> = vec![vec![]];
    for line in stacks {
        let line_chars: Vec<char> = line.chars().collect();
        let mut temp: Vec<char> = vec![];
        for i in 0..width {
            if (i - 1) % 4 == 0 {
                temp.push(line_chars[i as usize]);
            }
        }
        intermediate.push(temp);
    }
    // return vec![vec![]];
    intermediate[1..].to_vec()
}

fn print_stacks(stacks: Vec<&str>) {
    for line in stacks {
        println!("{:}", line);
    }
}

fn print_gathered_stacks(gathered_stacks: Vec<Vec<char>>) {
    for line in gathered_stacks {
        println!("{:?}", line);
    }
}

fn print_steps(steps: Vec<&str>) {
    for line in steps {
        println!("{:}", line);
    }
}

fn print_parsed_steps(parsed_steps: Vec<Vec<i32>>) {
    for line in parsed_steps {
        println!("{:?}", line);
    }
}

fn parse_step(step: &str) -> Vec<i32> {
    let words: Vec<&str> = step.split_whitespace().collect();
    if words.len() >= 6 {
        [words[1], words[3], words[5]]
            .iter()
            .map(|s| s.parse::<i32>().expect("Failed to convert word to i32"))
            .collect()
    } else {
        panic!("Not enough word in {:?}", words)
    }
}

fn parse_steps(steps: Vec<&str>) -> Vec<Vec<i32>> {
    let mut output: Vec<Vec<i32>> = vec![];
    for line in steps {
        output.push(parse_step(line));
    }
    output
}
