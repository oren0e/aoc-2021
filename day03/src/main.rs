use std::fs;

type Report = Vec<String>;

fn main() {
    // read the data
    let data = fs::read_to_string("data/real01.txt").expect("Unable to read file");
    let split = data.split("\n");
    let report: Vec<String> = split.map(|x| x.to_string()).collect();

    // part1
    println!(
        "Part 1 answer: {:?}",
        get_gamma_rate(report.clone()) * get_epsilon_rate(report.clone())
    );

    // part 2
    println!(
        "Part 2 answer: {:?}",
        find_oxygen_value(report.clone()) * find_co2_value(report)
    );
}

fn find_oxygen_value(report: Report) -> i32 {
    let mut report_copy = report.clone();
    for i in 0..report.len() {
        let report_t = transpose_report(report_copy.clone());
        report_copy = get_new_report_input_oxygen(report_copy.clone(), report_t, i);
        if report_copy.len() == 1 {
            break;
        }
    }

    i32::from_str_radix(&*report_copy[0], 2).expect("Could not convert from radix")
}

fn find_co2_value(report: Report) -> i32 {
    let mut report_copy = report.clone();
    for i in 0..report.len() {
        let report_t = transpose_report(report_copy.clone());
        report_copy = get_new_report_input_co2(report_copy.clone(), report_t, i);
        if report_copy.len() == 1 {
            break;
        }
    }

    i32::from_str_radix(&*report_copy[0], 2).expect("Could not convert from radix")
}

fn get_new_report_input_co2(report: Report, report_t: Report, position: usize) -> Report {
    let mut new_report: Report = vec![];
    if position < report_t.len() {
        let mask = report_t[position].clone();
        let most_common_bit = find_least_common_bit(mask, Some("co2"));
        for item in report {
            if item
                .chars()
                .nth(position)
                .expect("char at position not found")
                == most_common_bit
                    .chars()
                    .nth(0)
                    .expect("char at index 0 not found")
            {
                new_report.push(item.clone());
            }
        }
        new_report
    } else {
        panic!("Index out of bounds");
    }
}

fn get_new_report_input_oxygen(report: Report, report_t: Report, position: usize) -> Report {
    let mut new_report: Report = vec![];
    if position < report_t.len() {
        let mask = report_t[position].clone();
        let most_common_bit = find_most_common_bit(mask, Some("oxygen"));
        for item in report {
            if item
                .chars()
                .nth(position)
                .expect("char at position not found")
                == most_common_bit
                    .chars()
                    .nth(0)
                    .expect("char at index 0 not found")
            {
                new_report.push(item.clone());
            }
        }
        new_report
    } else {
        panic!("Index out of bounds");
    }
}

fn transpose_report(report: Report) -> Report {
    let mut report_t = vec![];
    for position in 0..report.first().expect("empty array").len() {
        let mut new_string = String::new();
        for (_, item) in report.iter().enumerate() {
            for (c, charr) in item.chars().enumerate() {
                if position == c {
                    new_string.push(charr)
                }
            }
        }
        report_t.push(new_string);
    }
    report_t
}

fn find_most_common_bit(s: String, part2: Option<&str>) -> String {
    let occurences0 = s.matches("0").count();
    let occurences1 = s.matches("1").count();
    if occurences1 > occurences0 {
        return "1".to_string();
    } else if occurences0 > occurences1 {
        return "0".to_string();
    } else {
        // for part 2
        match part2 {
            Some("oxygen") => "1".to_string(),
            Some("co2") => "0".to_string(),
            Some(_) => panic!("Invalid option"),
            None => panic!("Must enter one of (oxygen, co2)"),
        }
    }
}

fn find_least_common_bit(s: String, part2: Option<&str>) -> String {
    let occurences0 = s.matches("0").count();
    let occurences1 = s.matches("1").count();
    if occurences1 < occurences0 {
        return "1".to_string();
    } else if occurences0 < occurences1 {
        return "0".to_string();
    } else {
        // for part 2
        match part2 {
            Some("oxygen") => "1".to_string(),
            Some("co2") => "0".to_string(),
            Some(_) => panic!("Invalid option"),
            None => panic!("Must enter one of (oxygen, co2)"),
        }
    }
}

fn get_gamma_rate(report: Vec<String>) -> i32 {
    let report_t = transpose_report(report);
    let mut gama_rate_binary = String::new();
    for item in report_t {
        gama_rate_binary.push(
            find_most_common_bit(item, None)
                .chars()
                .nth(0)
                .expect("char at indedx 0 not found"),
        )
    }
    i32::from_str_radix(&*gama_rate_binary, 2).expect("Could not convert from radix")
}

fn get_epsilon_rate(report: Vec<String>) -> i32 {
    let report_t = transpose_report(report);
    let mut epsilon_rate_binary = String::new();
    for item in report_t {
        epsilon_rate_binary.push(
            find_least_common_bit(item, None)
                .chars()
                .nth(0)
                .expect("char at indedx 0 not found"),
        )
    }
    i32::from_str_radix(&*epsilon_rate_binary, 2).expect("Could not convert from radix")
}
