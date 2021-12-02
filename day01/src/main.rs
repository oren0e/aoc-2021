use std::fs;

fn main() {
    let data = fs::read_to_string("data/real01.txt").expect("Unable to read file");
    let split = data.split("\n");

    let depths: Vec<String> = split.map(|x| x.to_string()).collect();

    // part 1
    println!("{:?}", part1(depths.clone()));

    // part 2
    let mut sums = vec![];
    for (i, item) in depths.iter().enumerate() {
        if item == &"" {
            continue;
        }
        if (i + 2) >= depths.len() {
            break;
        }
        let summed = (depths[i].parse::<i32>().expect("parsing error")
            + depths[i + 1].parse::<i32>().expect("parsing error")
            + depths[i + 2].parse::<i32>().expect("parsing error"))
        .to_string();
        sums.push(summed);
    }
    println!("{:?}", part1(sums))
}

fn part1(depths: Vec<String>) -> usize {
    let mut changes = vec![];
    for (i, item) in depths.iter().enumerate() {
        if item == &"" {
            continue;
        }
        if i != 0 {
            if depths[i - 1].parse::<i32>().expect("parsing error")
                < depths[i].parse::<i32>().expect("parsing error")
            {
                changes.push("increased")
            } else {
                changes.push("decreased")
            }
        }
    }
    changes.iter().filter(|&n| *n == "increased").count()
}
