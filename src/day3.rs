use itertools::Itertools;

#[aoc(day3, part1)]
pub fn day3_part1(input: &str) -> i32 {
    let mut total = 0;
    for l in input.lines() {
        let (start, end) = l.split_at(l.len() / 2);
        let start = bytevec(&start);
        let end = bytevec(&end);

        let mut s = 0;
        let mut e = 0;

        let mut same = 0;
        while s != start.len() || e != end.len() {
            if start[s] < end[e] {
                s += 1;
            } else if start[s] > end[e] {
                e += 1;
            } else {
                same = *start[s];
                break;
            }
        }
        total += bytevalue(same);
    }
    total
}

pub fn bytevalue(b: u8) -> i32 {
        let mut value = b - 64;
        if value > 26 {
            value = value - 32;
        } else {
            value = value + 26
        }
        value as i32
}

pub fn bytevec(input: &str) -> Vec<&u8> {
    input.as_bytes().iter().sorted().collect::<Vec<&u8>>()
}

#[aoc(day3, part2)]
pub fn day3_part2(input: &str) -> i32 {
    let mut lines = input.lines();
    let mut total = 0;
    loop {
        let (line1, line2, line3) = (lines.next(), lines.next(), lines.next());
        if line1.is_none() { break }
        let (line1, line2, line3) = (bytevec(line1.unwrap()), bytevec(line2.unwrap()), bytevec(line3.unwrap()));

        let mut l1 = 0;
        let mut l2 = 0;
        let mut l3 = 0;

        while l1 != line1.len() && l2 != line2.len() && l3 != line3.len() {
            if line1[l1] == line2[l2] && line1[l1] == line3[l3] {
                total += bytevalue(*line1[l1]);
                break;
            }
            if line1[l1] < line2[l2] || line1[l1] < line3[l3] {
                l1 += 1;
            } else if line2[l2] < line1[l1] || line2[l2] < line3[l3] {
                l2 += 1;
            } else if line3[l3] < line1[l1] || line3[l3] < line2[l2] {
                l3 += 1;
            }
        }
    }
    total
}