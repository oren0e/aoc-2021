use std::fs;

fn main() {
    // read the data
    let data = fs::read_to_string("data/real01.txt").expect("Unable to read file");
    let directions = data
        .split("\n")
        .map(|s| s.split_at(s.find(" ").expect("Could not find splitting character")))
        .map(|(k, v)| Direction {
            direction: k.to_string(),
            amount: v[1..].parse::<i32>().expect("parsing error"),
        })
        .collect::<Vec<Direction>>();
    // part 1
    println!("{:?}", part1(directions.clone()));
    // part 2
    println!("{:?}", part2(directions));
}

#[derive(Debug, Clone)]
struct Direction {
    direction: String,
    amount: i32,
}

fn part1(directions: Vec<Direction>) -> i32 {
    let mut start = (0, 0); // (horizontal, depth)
    for item in &*directions {
        if item.direction == "forward" {
            start.0 += item.amount;
        } else if item.direction == "up" {
            start.1 -= item.amount;
        } else if item.direction == "down" {
            start.1 += item.amount;
        } else {
            panic!("Unknown direction...");
        }
    }
    return start.0 * start.1;
}

fn part2(directions: Vec<Direction>) -> i32 {
    let mut start = (0, 0, 0); // (horizontal, depth, aim)
    for item in &*directions {
        if item.direction == "forward" {
            start.0 += item.amount;
            start.1 += (item.amount * start.2);
        } else if item.direction == "up" {
            start.2 -= item.amount;
        } else if item.direction == "down" {
            start.2 += item.amount;
        } else {
            panic!("Unknown direction...")
        }
    }
    return start.0 * start.1;
}
