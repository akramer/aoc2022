use std::str::Lines;
use std::rc::Rc;
use std::cell::RefCell;


#[aoc(day7, part1)]
pub fn day7_part1(input: &str) -> i32 {
    let mut line_iterator = input.lines();
    let total = Rc::new(RefCell::new(0));
    commandloop(&mut line_iterator, Rc::clone(&total));
    dbg!(total);
    0
}

pub fn commandloop(lines: &mut Lines, total: Rc<RefCell<i32>>) -> i32 {
    let mut inside_total = 0;
    loop {
        let l = lines.next();
        if l.is_none() {
              if inside_total < 100000 {
                *total.borrow_mut() += inside_total;
              }
              return inside_total;
        }
        let l = l.unwrap();
        let c = l.split(" ").collect::<Vec<&str>>();
        if c[0] == "$" {
            if c[1] == "cd" {
                if c[2] == ".." {
                    if inside_total < 100000 {
                        *total.borrow_mut() += inside_total;
                    }
                    return inside_total
                } 
                let t = commandloop(lines, Rc::clone(&total));
                inside_total += t;
            }
        } else {
            if c[0] != "dir" {
                inside_total += c[0].parse::<i32>().unwrap();
            }
        }
    }
}


#[aoc(day7, part2)]
pub fn day7_part2(input: &str) -> i32 {
    let mut line_iterator = input.lines();
    let total = Rc::new(RefCell::new(0));
    let absolute = commandloop(&mut line_iterator, Rc::clone(&total));
    let mut line_iterator = input.lines();
    let min_to_free = 30000000 - (70000000 - absolute);
    dbg!(min_to_free);
    let to_free = Rc::new(RefCell::new(i32::MAX));
    commandloop2(&mut line_iterator, min_to_free, Rc::clone(&to_free));
    dbg!(Rc::clone(&to_free));
    0
}

pub fn commandloop2(lines: &mut Lines, min_to_free:i32, to_free: Rc<RefCell<i32>>) -> i32 {
    let mut inside_total = 0;
    loop {
        let l = lines.next();
        if l.is_none() {
            if inside_total < *to_free.borrow_mut() && inside_total > min_to_free {
                *to_free.borrow_mut() = inside_total;
            }
            println!("{} is final inside_total", inside_total);
              return inside_total;
        }
        let l = l.unwrap();
        let c = l.split(" ").collect::<Vec<&str>>();
        if c[0] == "$" {
            if c[1] == "cd" {
                if c[2] == ".." {
                    if inside_total < *to_free.borrow_mut() && inside_total > min_to_free {
                        *to_free.borrow_mut() = inside_total;
                    }
                    println!("{} is inside_total", inside_total);
                    return inside_total;
                } 
                let t = commandloop2(lines, min_to_free, Rc::clone(&to_free));
                inside_total += t;
            }
        } else {
            if c[0] != "dir" {
                inside_total += c[0].parse::<i32>().unwrap();
            }
        }
    }
}