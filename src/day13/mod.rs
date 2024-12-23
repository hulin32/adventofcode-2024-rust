use regex::Regex;
use std::collections::{HashMap};

pub struct Day13;

impl Day13 {
    pub fn filter_data(&self, input: &str) -> Vec<HashMap<char, (i64, i64)>> {
        let mut data: Vec<HashMap<char, (i64, i64)>> = vec![];
        let mut data_item: HashMap<char, (i64, i64)> = HashMap::new();
        input.lines().for_each(|line| {
            if line.is_empty() {
                data.push(data_item.clone());
                data_item = HashMap::new();
            } else {
                let re = Regex::new(r"(\d+)[^0-9]+(\d+)").unwrap();
                let caps = re.captures(line).unwrap();
                let x = caps.get(1).unwrap().as_str().parse::<i64>().unwrap();
                let y = caps.get(2).unwrap().as_str().parse::<i64>().unwrap();
                if line.contains("Button A") {
                    data_item.insert('A', (x, y));
                }
                if line.contains("Button B") {
                    data_item.insert('B', (x, y));
                }
                if line.contains("Prize:") {
                    data_item.insert('R', (x, y));
                }
            }
        });
        data.push(data_item.clone());
        data
    }

    fn first_part(&self, inputs: &str) -> i64 {
        let data = self.filter_data(inputs);
        data.iter()
            .map(|cal| {
                let (ax, ay) = cal.get(&'A').unwrap();
                let (bx, by) = cal.get(&'B').unwrap();
                let (rx, ry) = cal.get(&'R').unwrap();
                let mut a_count = 0;
                let mut b_count = 0;
                let mut found = false;
                let mut found_result = vec![];
                loop {
                    let left_x = rx - a_count * ax;
                    let left_y = ry - a_count * ay;
                    if left_x < 0 || a_count > 100 {
                        break;
                    }

                    let tmp_b_count = left_x as f64 / *bx as f64;

                    if left_y == (*by as f64 * tmp_b_count) as i64 && tmp_b_count <= 100.0 {
                        found = true;
                        b_count = tmp_b_count as i64;
                        found_result.push(a_count * 3 + b_count);
                    }
                    a_count += 1;
                }
                if found {
                    *found_result.iter().min().unwrap()
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn second_part(&self, input: &str) -> i64 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::day13::Day13;

    #[test]
    fn first_part_test() {
        assert_eq!(Day13.first_part(include_str!("day13_input_test.txt")), 480);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day13.first_part(include_str!("day13_input.txt")), 384157);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(
            Day13.second_part(include_str!("day13_input_test.txt")),
            1206
        );
    }

    #[test]
    fn second_part() {
        assert_eq!(Day13.second_part(include_str!("day13_input.txt")), 811148);
    }
}
