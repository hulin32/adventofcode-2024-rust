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
        let section_pattern = r"don't\(\)|do\(\)|mul\(\d+,\s*\d+\)";
        let section_re = Regex::new(section_pattern).unwrap();

        // Find all don't() mul(:num1, :num2) do() sections
        let mut sum = 0;
        let mut enabled = true;
        for section_caps in section_re.captures_iter(input) {
            let section_text = &section_caps[0];
            if section_text == "don't()" {
                enabled = false;
                continue;
            } else if section_text == "do()" {
                enabled = true;
                continue;
            }
            if enabled {
                // calculate mul(:num1, :num2) sections with first_part fn
                sum += self.first_part(section_text);
            }
        }
        sum
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
        assert_eq!(Day3.second_part(include_str!("day3_input.txt")), 90772405);
    }
}
