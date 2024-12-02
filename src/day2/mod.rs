pub struct Day2;

impl Day2 {
    pub fn first_part(input: &str) -> i32 {
        input
            .lines()
            .filter(|l| {
                let mut nums: Vec<i32> = l
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                if nums.first().unwrap() < nums.last().unwrap() {
                    nums.reverse();
                }

                !nums.windows(2).any(|w| {
                    let cal = w[0] - w[1];
                    !(1..=3).contains(&cal)
                })
            })
            .count() as i32
    }

    pub fn second_part(input: &str) -> i32 {
        input
            .lines()
            .filter(|l| {
                let mut nums: Vec<i32> = l
                    .split_whitespace()
                    .map(|s| s.parse::<i32>().unwrap())
                    .collect();
                if nums.first().unwrap() < nums.last().unwrap() {
                    nums.reverse();
                }

                let not_sufficient: Vec<i32> = nums
                    .windows(2)
                    .map(|w| w[0] - w[1])
                    .filter(|cal| !(1..=3).contains(cal))
                    .collect();

                let mut res = false;
                if not_sufficient.is_empty() {
                    res = true
                } else if not_sufficient.len() == 1 {
                    res = not_sufficient.first().unwrap().abs() <= 3
                } else if not_sufficient.len() == 2 {
                    res = not_sufficient.iter().map(|n| n.abs()).sum::<i32>() <= 3
                } else {
                    res = false
                }

                res
            })
            .count() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::Day2;

    #[test]
    fn first_part_test() {
        assert_eq!(Day2::first_part(include_str!("day2_input_test1.txt")), 2);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day2::first_part(include_str!("day2_input.txt")), 631);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day2::second_part(include_str!("day2_input_test2.txt")), 4);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day2::second_part(include_str!("day2_input.txt")), 22588371);
    }
}
