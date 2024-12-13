pub struct Day11;

impl Day11 {
    pub fn first_part(&self, input: &str, times: usize) -> i64 {
        let mut inputs: Vec<i64> = input
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|c| c.parse::<i64>().unwrap())
            .collect();
        for _ in 0..times {
            let mut new_inputs = vec![];
            inputs.iter().for_each(|&x| {
                if x == 0 {
                    new_inputs.push(1);
                } else if x.to_string().len() % 2 == 0 {
                    let mid = x.to_string().len() / 2;
                    new_inputs.push(x.to_string()[..mid].parse::<i64>().unwrap());
                    new_inputs.push(x.to_string()[mid..].parse::<i64>().unwrap());
                } else {
                    new_inputs.push(x * 2024);
                }
            });
            inputs = new_inputs;
        }
        inputs.len() as i64
    }

    pub fn second_part(&self, input: &str) -> String {
        "1".to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::Day11;

    #[test]
    fn first_part_test() {
        assert_eq!(
            Day11.first_part(include_str!("day11_input_test.txt"), 25),
            55312
        );
    }

    #[test]
    fn first_part() {
        assert_eq!(
            Day11.first_part(include_str!("day11_input.txt"), 25),
            183620
        );
    }

    #[test]
    fn second_part_test() {
        assert_eq!(
            Day11.second_part(include_str!("day11_input_test.txt")),
            "1 2024 1 0 9 9 2021976"
        );
    }

    #[test]
    fn second_part() {
        assert_eq!(
            Day11.second_part(include_str!("day11_input.txt")),
            "1 2024 1 0 9 9 2021976"
        );
    }
}
