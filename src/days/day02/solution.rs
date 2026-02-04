pub fn run(input: &str) {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();

    // println!("{:?}", lines);
    let mut total_score_part1: i32 = 0;
    for item in lines.clone() {
        let round_score: i32 = calculate_round_score_part1(item);
        // println!("round score: {}", round_score);
        total_score_part1 += round_score;
    }
    // println!("part1: {}", total_score);

    let mut total_score_part2: i32 = 0;

    for item in lines {
        let round_score: i32 = calculate_round_score_part2(item);
        total_score_part2 += round_score;
    }
    println!("part2: {}", total_score_part2);
}

fn calculate_round_score_part2(round: &str) -> i32 {
    let input_data: Vec<&str> = round.split_whitespace().collect();
    let move_enemy = get_move_part1(input_data[0]);
    let round_result = get_move_part2(input_data[1]);
    let move_player = get_move_player_part2(&move_enemy, &round_result);
    // println!(
    //     "enemy move: {}, round result: {}, player move: {}",
    //     move_enemy, round_result, move_player
    // );
    let move_score = get_score_part1(&move_player);
    let round_outcome_score = get_round_outcome_score_part2(&round_result);
    move_score + round_outcome_score
}

fn get_round_outcome_score_part2(input: &str) -> i32 {
    match input {
        "win" => 6,
        "lose" => 0,
        "draw" => 3,
        _ => panic!("KeyError: key {} does not exist!", input),
    }
}

fn get_move_player_part2(move_enemy: &str, round_result: &str) -> String {
    match move_enemy {
        "rock" => match round_result {
            "win" => "paper".to_string(),
            "lose" => "scissor".to_string(),
            "draw" => "rock".to_string(),
            _ => panic!("invalid round result: {}", round_result),
        },

        "paper" => match round_result {
            "win" => "scissor".to_string(),
            "lose" => "rock".to_string(),
            "draw" => "paper".to_string(),
            _ => panic!("invalid round result: {}", round_result),
        },

        "scissor" => match round_result {
            "win" => "rock".to_string(),
            "lose" => "paper".to_string(),
            "draw" => "scissor".to_string(),
            _ => panic!("invalid round result: {}", round_result),
        },

        _ => panic!("invalid enemy move: {}", move_enemy),
    }
}

fn get_move_part2(input: &str) -> String {
    match input {
        "X" => "lose".to_string(),
        "Y" => "draw".to_string(),
        "Z" => "win".to_string(),
        _ => panic!("KeyError: key {} does not exist!", input),
    }
}

fn calculate_round_score_part1(round: &str) -> i32 {
    let moves: Vec<&str> = round.split_whitespace().collect();
    let move_enemy = get_move_part1(moves[0]);
    let move_player = get_move_part1(moves[1]);

    // println!("move: {:?}", moves);
    // println!("my move: {}, enemy move: {}", move_player, move_enemy);
    let round_result: i32 = get_result_part1(&move_enemy, &move_player);
    let move_score: i32 = get_score_part1(&move_player);

    round_result + move_score
}

fn get_score_part1(input: &str) -> i32 {
    match input {
        "rock" => 1,
        "paper" => 2,
        "scissor" => 3,
        _ => panic!("KeyError: key {} does not exist!", input),
    }
}

fn get_move_part1(input: &str) -> String {
    match input {
        "X" => "rock".to_string(),
        "A" => "rock".to_string(),
        "Y" => "paper".to_string(),
        "B" => "paper".to_string(),
        "Z" => "scissor".to_string(),
        "C" => "scissor".to_string(),
        _ => panic!("KeyError: key {} does not exist!", input),
    }
}

fn get_result_part1(move_enemy: &str, move_player: &str) -> i32 {
    match move_player {
        "rock" => match move_enemy {
            "rock" => 3,
            "paper" => 0,
            "scissor" => 6,
            _ => panic!("invalid enemy move: {}", move_enemy),
        },
        "paper" => match move_enemy {
            "rock" => 6,
            "paper" => 3,
            "scissor" => 0,
            _ => panic!("invalid enemy move: {}", move_enemy),
        },
        "scissor" => match move_enemy {
            "rock" => 0,
            "paper" => 6,
            "scissor" => 3,
            _ => panic!("invalid enemy move: {}", move_enemy),
        },
        _ => panic!("invalid player move: {}", move_player),
    }
}
