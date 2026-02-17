pub fn run(input: &str) {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    // println!("{:?}", lines);
    let mut total_full_range: i32 = 0;
    let mut total_overlap: i32 = 0;
    for line in lines {
        let pairs = get_pairs(line);
        if full_range_check(pairs[0], pairs[1]) {
            total_full_range += 1;
        }
        // let pairs = get_pairs(line);
        if overlap_check(pairs[0], pairs[1]) {
            total_overlap += 1;
        }
    }
    println!("total_full_range: {}", total_full_range);
    println!("total_overlap: {}", total_overlap);
}

fn get_pairs(line: &str) -> Vec<&str> {
    line.split(",").collect()
}

fn full_range_check(range1: &str, range2: &str) -> bool {
    let range1_vec: Vec<i32> = range1
        .split("-")
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    let range2_vec: Vec<i32> = range2
        .split("-")
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    if (range1_vec[0] <= range2_vec[0]) && (range1_vec[1] >= range2_vec[1]) {
        true
    } else {
        (range1_vec[0] >= range2_vec[0]) && (range1_vec[1] <= range2_vec[1])
    }
}

fn overlap_check(range1: &str, range2: &str) -> bool {
    let range1_vec: Vec<i32> = range1
        .split("-")
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    let range2_vec: Vec<i32> = range2
        .split("-")
        .map(|c| c.parse::<i32>().unwrap())
        .collect();
    !((range1_vec[1] < range2_vec[0]) || (range2_vec[1] < range1_vec[0]))
}
