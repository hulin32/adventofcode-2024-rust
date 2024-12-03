use fancy_regex::Regex;

pub struct Day3;

impl Day3 {
    pub fn first_part(&self, input: &str) -> i32 {
        let pattern = r"mul\((\d+),(\d+)\)";
        let re = Regex::new(pattern).unwrap();
        re.captures_iter(input)
            .map(|caps| {
                let res = caps.unwrap();
                res[1].parse::<i32>().unwrap() * res[2].parse::<i32>().unwrap()
            })
            .sum()
    }

    pub fn second_part(&self, input: &str) -> i32 {
        let section_pattern = r"don't\(\)((?:(?!don't\(\)|do\(\)).)*?)do\(\)";
        let section_re = Regex::new(section_pattern).unwrap();

        // Find all don't()...do() sections
        let mut sum = 0;
        for section_caps in section_re.captures_iter(input) {
            // Get the full text between don't() and do()
            let section_text = &section_caps.unwrap()[0];
            println!("section_text: {}", section_text);
            sum += self.first_part(section_text);
        }
        self.first_part(input) - sum
    }
}

#[cfg(test)]
mod tests {
    use crate::day3::Day3;

    #[test]
    fn first_part_test() {
        assert_eq!(Day3.first_part(include_str!("day3_input_test1.txt")), 161);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day3.first_part(include_str!("day3_input.txt")), 182780583);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day3.second_part(include_str!("day3_input_test2.txt")), 48);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day3.second_part(include_str!("day3_input.txt")), 665);
    }
}
