use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};
use std::fmt::format;

pub struct Day7;

#[derive(Debug, PartialEq, Clone)]
enum Operation {
    Add,
    Multiply,
    Combine
}

impl Day7 {
    fn calculate(&self, input: &str, use_comibine: bool) -> i64 {
        let mut map = HashMap::new();
        input.lines().for_each(|l| {
            let s = l.split(":").collect::<Vec<&str>>();
            let result = s[0].trim().parse::<i64>().unwrap();
            let digits: Vec<i64> = s[1]
                .split(" ")
                .filter(|c| !c.is_empty())
                .map(|c| c.parse::<i64>().unwrap())
                .collect();
            map.insert(result, digits);
        });
        let mut total_result = 0;
        map.iter().for_each(|(k, v)| {
            let operator_count = v.len() - 1;
            let mut operators = vec![
                vec![Operation::Add],
                vec![Operation::Multiply],
            ];
            if use_comibine {
                operators.push(vec![Operation::Combine]);
            }
            for _ in 0..operator_count - 1 {
                let mut new_operators = vec![];
                operators.iter().for_each(|op| {
                    let mut new_op = op.clone();
                    new_op.push(Operation::Add);
                    new_operators.push(new_op.clone());

                    new_op = op.clone();
                    new_op.push(Operation::Multiply);
                    new_operators.push(new_op.clone());

                    if use_comibine {
                        new_op = op.clone();
                        new_op.push(Operation::Combine);
                        new_operators.push(new_op.clone());
                    }

                });
                operators = new_operators;
            }

            for operator_list in operators {
                let mut result = v[0];
                for i in 0..operator_count {
                    match operator_list[i] {
                        Operation::Add => result += v[i + 1],
                        Operation::Multiply => result *= v[i + 1],
                        Operation::Combine => result = format!("{}{}", result, v[i + 1]).parse::<i64>().unwrap()
                    }
                }
                if result == *k {
                    total_result += result;
                    break;
                }
            }
        });
        total_result
    }
    pub fn first_part(&self, input: &str) -> i64 {
        self.calculate(input, false)
    }
    pub fn second_part(&self, input: &str) -> i64 {
        self.calculate(input, true)
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::Day7;

    #[test]
    fn first_part_test() {
        assert_eq!(Day7.first_part(include_str!("day7_input_test.txt")), 3749);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day7.first_part(include_str!("day7_input.txt")), 28730327770375);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day7.second_part(include_str!("day7_input_test.txt")), 11387);
    }

    #[test]
    fn second_part() {
        // TODO slow 30s
        assert_eq!(Day7.second_part(include_str!("day7_input.txt")), 424977609625985);
    }
}
