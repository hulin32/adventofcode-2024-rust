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

                let mut not_sufficient_idxs = vec![];
                let not_sufficient: Vec<i32> = nums
                    .windows(2)
                    .map(|w| w[0] - w[1])
                    .enumerate()
                    .filter(|(i, cal)| {
                        if !(1..=3).contains(cal) {
                            not_sufficient_idxs.push(*i);
                            true
                        } else {
                            false
                        }
                    })
                    .map(|(_, cal)| cal)
                    .collect();

                if not_sufficient.is_empty() {
                    true
                } else {
                    not_sufficient_idxs
                        .iter()
                        .flat_map(|&idx| vec![idx, idx + 1])
                        .any(|i| {
                            let mut nums = nums.clone();
                            nums.remove(i);
                            nums.windows(2)
                                .map(|w| w[0] - w[1])
                                .filter(|cal| !(1..=3).contains(cal))
                                .collect::<Vec<_>>()
                                .is_empty()
                        })
                }
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