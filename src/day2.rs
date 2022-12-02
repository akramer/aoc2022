#[aoc(day2, part1)]
pub fn day2_part1(input: &str) -> i32 {
    let mut score = 0;
    for l in input.lines() {
        let mut play = l.split_whitespace();
        let them = rps_value(play.next().unwrap());
        let me = rps_value(play.next().unwrap());
        score += rps_score1(them, me);
    }
    return score;
}

pub fn rps_value(m: &str) -> i32 {
    match m {
        "A" => 0,
        "B" => 1,
        "C" => 2,
        "X" => 0,
        "Y" => 1,
        "Z" => 2,
        _ => panic!("How'd we get here?"),
    }
}

pub fn rps_score1(them: i32, me: i32) -> i32 {
    let mut score = me + 1;

    if me == them {
        score += 3;
    } else if (them + 1) % 3 == me {
        score += 6;
    }
    score
}

#[aoc(day2, part2)]
pub fn day2_part2(input: &str) -> i32 {
    let mut score = 0;
    for l in input.lines() {
        let mut play = l.split_whitespace();
        let them = rps_value(play.next().unwrap());
        let outcome = play.next().unwrap();
        score += rps_score2(them, outcome);
    }
    return score;
}

pub fn rps_score2(them: i32, outcome: &str) -> i32 {
    let score = match outcome {
        "Z" => 6 + ((them + 1) % 3 + 1),
        "Y" => 3 + them + 1,
        "X" => 0 + ((them - 1).rem_euclid(3) + 1),
        _ => panic!("invalid"),
    };
    return score;
}
