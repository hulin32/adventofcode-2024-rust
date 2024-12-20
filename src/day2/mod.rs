pub struct Day2;

impl Day2 {
    fn parse_line(&self, l: &str) -> Vec<i32> {
        let mut nums: Vec<i32> = l
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();
        if nums.first().unwrap() < nums.last().unwrap() {
            nums.reverse();
        }
        nums
    }

    fn is_safe(&self, nums: &[i32]) -> bool {
        nums.windows(2)
            .map(|w| w[0] - w[1])
            .all(|cal| (1..=3).contains(&cal))
    }

    pub fn first_part(&self, input: &str) -> usize {
        input
            .lines()
            .filter(|&l| self.is_safe(&self.parse_line(l)))
            .count()
    }

    pub fn second_part(&self, input: &str) -> usize {
        input
            .lines()
            .filter(|l| {
                let nums = self.parse_line(l);
                let not_sufficient_idxs: Vec<usize> = nums
                    .windows(2)
                    .map(|w| w[0] - w[1])
                    .enumerate()
                    .filter(|(_, cal)| !(1..=3).contains(cal))
                    .map(|(idx, _)| idx)
                    .collect();

                if not_sufficient_idxs.is_empty() {
                    return true;
                }

                // narrow down to Unsafe indexes
                not_sufficient_idxs
                    .iter()
                    .flat_map(|&idx| vec![idx, idx + 1])
                    .any(|i| {
                        let mut nums = nums.clone();
                        nums.remove(i);
                        self.is_safe(&nums)
                    })
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::Day2;

    #[test]
    fn first_part_test() {
        assert_eq!(Day2.first_part(include_str!("day2_input_test1.txt")), 2);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day2.first_part(include_str!("day2_input.txt")), 631);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day2.second_part(include_str!("day2_input_test2.txt")), 4);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day2.second_part(include_str!("day2_input.txt")), 665);
    }
}
