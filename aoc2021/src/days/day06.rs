use crate::base::Day;
use std::collections::VecDeque;

pub struct Day06;

impl Day for Day06 {
    type Parsed = Vec<usize>;
    type Output1 = u64;
    type Output2 = u64;

    fn day_num() -> usize {
        6
    }

    fn title() -> &'static str {
        "Lanterfish Growth"
    }

    fn parse(input: &str) -> Self::Parsed {
        input
            .split(',')
            .map(|x| x.parse().expect("Parsing error"))
            .collect()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        grow(data, 80)
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        grow(data, 256)
    }
}

fn grow(starting_population: Vec<usize>, permutations: usize) -> u64 {
    let mut counter = VecDeque::new();
    counter.extend([0u64; 9]);
    for internal_timer in starting_population {
        counter[internal_timer] += 1
    }

    for _ in 0..permutations {
        let c = counter
            .pop_front()
            .expect("Failed to pop from front of vecdeque");
        counter.push_back(c);
        counter[6] += c;
    }

    counter.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::InputSource;

    #[test]
    fn day06_part1_example() {
        Day06::test(
            InputSource::File {
                path: "data/day06_example.txt".to_string(),
            },
            Some(5934),
            None,
        )
    }

    #[test]
    fn day06_part1_real() {
        Day06::test(
            InputSource::File {
                path: "data/day06_real.txt".to_string(),
            },
            Some(352872),
            None,
        )
    }

    #[test]
    fn day06_part2_example() {
        Day06::test(
            InputSource::File {
                path: "data/day06_example.txt".to_string(),
            },
            None,
            Some(26984457539),
        )
    }

    #[test]
    fn day06_part2_real() {
        Day06::test(
            InputSource::File {
                path: "data/day06_real.txt".to_string(),
            },
            None,
            Some(1604361182149),
        )
    }
}
