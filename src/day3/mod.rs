use regex::Regex;

pub struct Day3;

impl Day3 {
    pub fn first_part(&self, input: &str) -> i32 {
        let pattern = r"mul\((\d+),(\d+)\)";
        let re = Regex::new(pattern).unwrap();
        re.captures_iter(input)
            .map(|caps| caps[1].parse::<i32>().unwrap() * caps[2].parse::<i32>().unwrap())
            .sum()
    }

    pub fn second_part(&self, input: &str) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::Day3;

    #[test]
    fn first_part_test() {
        assert_eq!(Day3.first_part(include_str!("day3_input_test.txt")), 161);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day3.first_part(include_str!("day3_input.txt")), 182780583);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day3.second_part(include_str!("day3_input_test.txt")), 4);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day3.second_part(include_str!("day3_input.txt")), 665);
    }
}
