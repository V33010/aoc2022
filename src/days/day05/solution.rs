pub fn run(input: &str) {
    let lines: Vec<&str> = input.lines().collect();

    println!("{:?}", lines);
    let (stacks, steps) = parse_input(lines);
    println!("stacks:");
    print_stacks(stacks.clone());
    println!("");
    println!("steps:");
    print_steps(steps.clone());
    println!("");
    let parsed_steps = parse_steps(steps);
    // println!("parsed_steps:");
    // print_parsed_steps(parsed_steps.clone());
    // println!("");
    let gathered_stacks = gather_stacks(stacks);
    // println!("gathered_stacks:");
    // print_gathered_stacks(gathered_stacks.clone());
    let inverted_stacks = invert_matrix(gathered_stacks);
    // println!("");
    //
    // println!("line in inverted_stacks:");
    // for line in inverted_stacks.clone() {
    //     println!("{:?}", line);
    // }
    let refactored_stacks = refactor_stacks(inverted_stacks);
    let output_matrix = comply_all_steps(refactored_stacks, parsed_steps);
    let tops = get_tops(output_matrix.clone());
    let tops_string = get_tops_string(tops.clone());
    // println!("line in output_matrix");
    // for line in output_matrix {
    //     println!("{:?}", line);
    // }
    // println!("tops: {:?}", tops);
    println!("tops_string: {}", tops_string);
}

fn get_tops(input_matrix: Vec<Vec<char>>) -> Vec<char> {
    let mut output: Vec<char> = vec![];
    for item in input_matrix {
        output.push(item[0]);
    }
    output
}

fn get_tops_string(tops: Vec<char>) -> String {
    tops.into_iter().collect()
}

fn remove_blanks(stack: Vec<char>) -> Vec<char> {
    let mut output_stack: Vec<char> = vec![];
    for item in stack {
        match item {
            ' ' => {}
            _ => output_stack.push(item),
        }
    }
    output_stack
}

fn refactor_stacks(inverted_stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut output_stacks: Vec<Vec<char>> = vec![];
    for item in inverted_stacks {
        output_stacks.push(remove_blanks(item));
    }
    output_stacks
}

fn comply_once(loc_from: i32, loc_to: i32, mut input_matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let loc_from_as_idx: usize = loc_from as usize;
    let loc_to_as_idx: usize = loc_to as usize;
    if !input_matrix[loc_from_as_idx].is_empty() {
        let popped_elem: char = input_matrix[loc_from_as_idx].remove(0);
        input_matrix[loc_to_as_idx].insert(0, popped_elem);
    }
    // for line in input_matrix.clone() {
    //     println!("{:?}", line);
    // }
    // println!("");
    input_matrix
}

fn comply_step(
    loc_from: i32,
    loc_to: i32,
    mut input_matrix: Vec<Vec<char>>,
    iters: i32,
) -> Vec<Vec<char>> {
    for _i in 0..iters {
        input_matrix = comply_once(loc_from, loc_to, input_matrix);
    }

    input_matrix
}

fn comply_all_steps(
    mut input_matrix: Vec<Vec<char>>,
    parsed_steps: Vec<Vec<i32>>,
) -> Vec<Vec<char>> {
    let length: i32 = parsed_steps.len() as i32;
    for i in 0..length {
        let i_as_idx = i as usize;
        let iters = parsed_steps[i_as_idx][0];
        let loc_from = parsed_steps[i_as_idx][1] - 1;
        let loc_to = parsed_steps[i_as_idx][2] - 1;
        input_matrix = comply_step(loc_from, loc_to, input_matrix, iters)
    }
    input_matrix
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
    // println!("height: {}", height);
    let width = input_matrix[0].len();
    // println!("width: {}", width);
    let mut output: Vec<Vec<char>> = vec![vec![]];
    for i in 0..width {
        let mut temp: Vec<char> = vec![];
        for j in 0..height {
            temp.push(input_matrix[j][i])
        }
        output.push(temp);
    }
    output[1..].to_vec()
}

fn gather_stacks(stacks: Vec<&str>) -> Vec<Vec<char>> {
    let width = stacks[0].len() as i32;
    let total_horizontal = (width + 1) / 3;
    // println!("total_horizontal: {}", total_horizontal);
    // println!("width: {}", width);
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
