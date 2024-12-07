use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

pub struct Day7;

#[derive(Debug, PartialEq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn move_step(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
    fn next_direction(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

impl Day7 {
    pub fn first_part(&self, input: &str) -> i32 {
        let mut map = HashMap::new();
        input.lines().for_each(|l| {
            let s = l.split(":").collect::<Vec<&str>>();
            let result = s[0].trim().parse::<i32>().unwrap();
            let digits: Vec<i32> = s[1]
                .split(" ")
                .filter(|c| !c.is_empty())
                .map(|c| c.parse::<i32>().unwrap())
                .collect();
            map.insert(result, digits);
        });
        map.iter().for_each(|(k, v)| {
            println!("{:?} {:?}", k, v);
        });
        1
    }
    pub fn second_part(&self, input: &str) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::day7::Day7;

    #[test]
    fn first_part_test() {
        assert_eq!(Day7.first_part(include_str!("day7_input_test.txt")), 41);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day7.first_part(include_str!("day7_input.txt")), 5145);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day7.second_part(include_str!("day7_input_test.txt")), 6);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day7.second_part(include_str!("day7_input.txt")), 6179);
    }
}
