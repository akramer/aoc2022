#[aoc(day1, part1)]
pub fn solve_part1(input: &str) -> i32 {
    let mut max = 0;
    let mut current_sum = 0;
    for l in input.lines() {
        if l == "" {
            if current_sum > max {
                max = current_sum;
            }
            current_sum = 0;
        } else {
            let c: i32 = l.parse().unwrap();
            current_sum += c;
        }
    }
    return max;
}

#[aoc(day1, part2, adam)]
pub fn solve_part2(input: &str) -> usize {
    let mut max = vec![0; 3];
    let mut current_sum = 0usize;
    for l in input.lines() {
        if l == "" {
            for i in max.iter_mut() {
                if current_sum > *i {
                    (*i, current_sum) = (current_sum, *i);
                }
            }
            current_sum = 0;
        } else {
            let c: usize = l.parse().unwrap();
            current_sum += c;
        }
    }
    return max.iter().sum();
}
