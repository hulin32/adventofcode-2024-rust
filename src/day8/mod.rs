use std::collections::{HashMap, HashSet};

pub struct Day8;

impl Day8 {
    pub fn first_part(&self, input: &str) -> i32 {
        let y_max = input.lines().count() as i32;
        let x_max = input.lines().next().unwrap().len() as i32;
        let mut antennas = HashMap::new();
        input.lines().enumerate().for_each(|(y, chars)| {
            chars.chars().enumerate().for_each(|(x, c)| {
                if c != '.' {
                    antennas
                        .entry(c)
                        .and_modify(|c: &mut Vec<(i32, i32)>| c.push((x as i32, y as i32)))
                        .or_insert(vec![(x as i32, y as i32)]);
                }

            });
        });
        let mut count = HashSet::new();
        antennas.into_iter().for_each(|(_c, ant)| {
            for i in 0..ant.len() - 1 {
                for j in i + 1..ant.len() {
                    let (x1, y1) = ant[i];
                    let (x2, y2) = ant[j];
                    let x = x2 - x1 ;
                    let y = y2 - y1;
                    let bottom = (x2 + x, y2 + y);
                    let top = (x1 - x, y1 - y);
                    if bottom.0 >= 0 && bottom.0 < x_max && bottom.1 >= 0 && bottom.1 < y_max {
                        count.insert(bottom);
                    }
                    if top.0 >= 0 && top.0 < x_max && top.1 >= 0 && top.1 < y_max {
                        count.insert(top);
                    }
                }
            }
        });
        count.len() as i32
    }

    pub fn second_part(&self, input: &str) -> i32 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::day8::Day8;

    #[test]
    fn first_part_test() {
        assert_eq!(Day8.first_part(include_str!("day8_input_test.txt")), 14);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day8.first_part(include_str!("day8_input.txt")), 1590491);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day8.second_part(include_str!("day8_input_test.txt")), 31);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day8.second_part(include_str!("day8_input.txt")), 22588371);
    }
}
