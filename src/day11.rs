use std::collections::VecDeque;
use std::fmt;

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Monkey")
            .field("items", &self.items)
            .field("test", &self.test)
            .field("true_offset", &self.true_offset)
            .field("false_offset", &self.false_offset)
            .field("inspections", &self.inspections)
            .finish()
    }
}
struct Monkey {
    items: VecDeque<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: i64,
    true_offset: usize,
    false_offset: usize,
    inspections: i64,
}

#[aoc(day11, part1)]
pub fn day11_part1(input: &str) -> i64 {
    let mut lines = input.lines();
    let mut monkeys = Vec::<Monkey>::new();
    loop {
        let mut m = Monkey {
            items: VecDeque::new(),
            operation: Box::new(|_i64| -> i64 { 0 }),
            test: 0,
            true_offset: 0,
            false_offset: 0,
            inspections: 0,
        };
        // ignore Monkey#
        let l = lines.next();
        if l.is_none() {
            break;
        }
        let mut starting = lines.next().unwrap().split(": ");
        let _ = starting.next().unwrap();
        let starting_data = starting.next().unwrap();
        let starting_data = starting_data.split(", ");
        for d in starting_data {
            dbg!(d);
            m.items.push_back(d.parse().unwrap())
        }
        // operation
        let mut op_string = lines.next().unwrap().split("= ");
        let _ = op_string.next().unwrap();
        let mut op_string = op_string.next().unwrap().split(" ");
        let _ = op_string.next().unwrap();
        let o = op_string.next().unwrap();
        let v = op_string.next().unwrap();
        if o == "+" {
            if v == "old" {
                m.operation = Box::new(|i| -> i64 { i + i })
            } else {
                let v = v.parse::<i64>().unwrap();
                m.operation = Box::new(move |i| -> i64 { i + v })
            }
        } else if o == "*" {
            if v == "old" {
                m.operation = Box::new(|i| -> i64 { i * i })
            } else {
                let v = v.parse::<i64>().unwrap();
                m.operation = Box::new(move |i| -> i64 { i * v })
            }
        }

        //test
        let mut test_string = lines.next().unwrap().split(' ');
        m.test = test_string.next_back().unwrap().parse().unwrap();
        //true monkey
        m.true_offset = lines
            .next()
            .unwrap()
            .split(' ')
            .next_back()
            .unwrap()
            .parse()
            .unwrap();
        // false monkey
        m.false_offset = lines
            .next()
            .unwrap()
            .split(' ')
            .next_back()
            .unwrap()
            .parse()
            .unwrap();
        // empty line
        _ = lines.next();
        monkeys.push(m);
    }
    for _ in 0..20 {
        do_iteration(&mut monkeys);
    }
    let mut top2: Vec<i64> = monkeys.iter().map(|m| m.inspections).collect();
    top2.sort();
    dbg!(monkeys);
    return top2[top2.len()-1] * top2[top2.len()-2];
}

fn do_iteration(monkeys: &mut Vec<Monkey>) {
    let mut n = 0;
    while n < monkeys.len() {
        while let Some(mut i) = monkeys[n].items.pop_front() {
            monkeys[n].inspections += 1;
            i = (monkeys[n].operation)(i);
            i /= 3;
            let offset;
            if i % monkeys[n].test == 0 {
                offset = monkeys[n].true_offset;
            } else {
                offset = monkeys[n].false_offset;
            }
            monkeys[offset].items.push_back(i)
        }
        n += 1;
    }
}
