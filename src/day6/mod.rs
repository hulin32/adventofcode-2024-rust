use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet};

pub struct Day6;

#[derive(Debug, PartialEq, Clone, Eq, Hash)]
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

impl Day6 {
    pub fn first_part(&self, input: &str) -> i32 {
        let mut map = HashSet::new();
        let mut x = 0;
        let mut y = 0;
        let mut start = (0, 0);
        input.lines().enumerate().for_each(|(i, l)| {
            x = (l.len() - 1) as i32;
            y = i as i32;
            let s = l.split("").filter(|c| !c.is_empty()).collect::<Vec<&str>>();
            s.iter().enumerate().for_each(|(j, c)| {
                if c == &"#" {
                    map.insert((j as i32, i as i32));
                }
                if c == &"^" {
                    start = (j as i32, i as i32);
                }
            });
        });
        let mut count = HashSet::new();
        let mut direction = Direction::Up;
        let mut current = start;
        while current.0 >= 0 && current.0 <= x && current.1 >= 0 && current.1 <= y {
            count.insert(current);
            let move_step = direction.move_step();
            let next = (current.0 + move_step.0, current.1 + move_step.1);
            if map.contains(&next) {
                direction = direction.next_direction();
            } else {
                current = next;
            }
        }
        count.len() as i32
    }
    pub fn second_part(&self, input: &str) -> i32 {
        let mut walls = HashMap::new();
        let mut x = 0;
        let mut y = 0;
        let mut start = (0, 0);
        input.lines().enumerate().for_each(|(i, l)| {
            x = (l.len() - 1) as i32;
            y = i as i32;
            let s = l.split("").filter(|c| !c.is_empty()).collect::<Vec<&str>>();
            s.iter().enumerate().for_each(|(j, c)| {
                if c == &"#" {
                    walls.insert((j as i32, i as i32), true);
                }
                if c == &"^" {
                    start = (j as i32, i as i32);
                }
            });
        });

        let mut walks = vec![];
        let mut direction = Direction::Up;
        let mut current = start;
        while current.0 >= 0 && current.0 <= x && current.1 >= 0 && current.1 <= y {
            if current != start {
                walks.push(current);
            }
            let move_step = direction.move_step();
            let next = (current.0 + move_step.0, current.1 + move_step.1);
            if walls.get(&next).is_some() {
                direction = direction.next_direction();
            } else {
                current = next;
            }
        }
        // put obstruction from latest to oldest, loop it
        // then remove one step, walk again
        // if one position is visited 4 times, then stop, its obstructed
        let mut count = HashSet::new();
        while let Some(change_to_wall) = walks.pop() {
            walls.insert(change_to_wall, true);
            let mut new_walks = HashMap::new();
            let mut direction = Direction::Up;
            let mut current = start;
            let mut valid_obstruction = false;
            while current.0 >= 0 && current.0 <= x && current.1 >= 0 && current.1 <= y {
                if new_walks.contains_key(&(current, direction.clone())) {
                    valid_obstruction = true;
                    break;
                }
                new_walks.insert((current, direction.clone()), true);
                let move_step = direction.move_step();
                let next = (current.0 + move_step.0, current.1 + move_step.1);
                if walls.contains_key(&next) {
                    direction = direction.next_direction();
                } else {
                    current = next;
                }
            }
            if valid_obstruction {
                count.insert(change_to_wall);
            }
            walls.remove(&change_to_wall);
        }
        count.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::day6::Day6;

    #[test]
    fn first_part_test() {
        assert_eq!(Day6.first_part(include_str!("day6_input_test.txt")), 41);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day6.first_part(include_str!("day6_input.txt")), 5145);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day6.second_part(include_str!("day6_input_test.txt")), 6);
    }

    #[test]
    fn second_part() {
        // TODO slow 30s
        assert_eq!(Day6.second_part(include_str!("day6_input.txt")), 1523);
    }
}
