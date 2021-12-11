use crate::base::Day;

pub struct DayXX;

impl Day for DayXX {
    type Parsed = usize;
    type Output1 = u32;
    type Output2 = u32;

    fn day_num() -> usize {
        todo!()
    }

    fn title() -> &'static str {
        todo!()
    }

    fn parse(input: &str) -> Self::Parsed {
        todo!()
    }

    fn part1(data: Self::Parsed) -> Self::Output1 {
        todo!()
    }

    fn part2(data: Self::Parsed) -> Self::Output2 {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::base::InputSource;

    #[test]
    fn dayXX_part1_example() {
        DayXX::test(
            InputSource::File {
                path: "data/dayXX_example.txt".to_string(),
            },
            Some(300),
            None,
        )
    }

    #[test]
    fn dayXX_part1_real() {
        DayXX::test(
            InputSource::File {
                path: "data/dayXX_real.txt".to_string(),
            },
            Some(300),
            None,
        )
    }

    #[test]
    fn dayXX_part2_example() {
        DayXX::test(
            InputSource::File {
                path: "data/dayXX_example.txt".to_string(),
            },
            None,
            Some(300),
        )
    }

    #[test]
    fn dayXX_part2_real() {
        DayXX::test(
            InputSource::File {
                path: "data/dayXX_real.txt".to_string(),
            },
            None,
            Some(300),
        )
    }
}
