use crate::base::Day;
use stats::median;

pub struct Day07;

impl Day for Day07 {
    type Parsed = Vec<f64>;
    type Output1 = f64;
    type Output2 = i64;

    fn day_num() -> usize {
        7
    }

    fn title() -> &'static str {
        "Crab Alignment"
    }

    fn parse(input: &str) -> Self::Parsed {
        input
            .split(',')
            .map(|x| x.parse().expect("Parsing Error"))
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        let m = median(data.clone().into_iter()).expect("Empty vector");
        data.iter().map(|x| f64::abs(x - m)).sum()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        let best_position = find_best_position(data.iter().map(|x| *x as i64).collect());
        total_number_of_cost_to_target(data.iter().map(|x| *x as i64).collect(), best_position)
    }
}

fn cost(num_steps: i64) -> i64 {
    (num_steps * (num_steps + 1)) / 2
}

fn total_number_of_cost_to_target(data: Vec<i64>, target: i64) -> i64 {
    data.iter().map(|x| cost(i64::abs(x - target))).sum()
}

fn find_best_position(data: Vec<i64>) -> i64 {
    let start: i64 = *data.iter().min().expect("min operation failed");
    let end: i64 = *data.iter().max().expect("max operation failed");
    (start..(end + 1))
        .min_by_key(|x| total_number_of_cost_to_target(data.clone(), *x))
        .expect("min by key failed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::InputSource;

    #[test]
    fn day07_part1_example() {
        Day07::test(
            InputSource::File {
                path: "data/day07_example.txt".to_string(),
            },
            Some(37.0),
            None,
        )
    }

    #[test]
    fn day07_part1_real() {
        Day07::test(
            InputSource::File {
                path: "data/day07_real.txt".to_string(),
            },
            Some(357353.0),
            None,
        )
    }

    #[test]
    fn day07_part2_example() {
        Day07::test(
            InputSource::File {
                path: "data/day07_example.txt".to_string(),
            },
            None,
            Some(168),
        )
    }

    #[test]
    fn day07_part2_real() {
        Day07::test(
            InputSource::File {
                path: "data/day07_real.txt".to_string(),
            },
            None,
            Some(104822130),
        )
    }
}
