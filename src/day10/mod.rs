use std::collections::HashMap;

pub struct Day10;

impl Day10 {
    fn calc_num_paths(&self, input: &str, unique: bool) -> usize {
        let mut trail_heads = vec![];
        let mut end_points = vec![];
        let mut map = HashMap::new();
        let mut max_x = 0;
        let mut max_y = 0;
        input.lines().enumerate().for_each(|(y, line)| {
            max_y = max_y.max(y);
            line.chars().enumerate().for_each(|(x, ch)| {
                max_x = max_x.max(x);
                let num = ch.to_string().parse::<i32>().unwrap();
                map.insert((x, y), num);
                if num == 0 {
                    trail_heads.push((x, y));
                }
                if num == 9 {
                    end_points.push((x, y));
                }
            });
        });

        let mut count = 0;
        for (x, y) in trail_heads {
            let mut paths = vec![(x, y)];
            for cur_num in 1..=9 {
                let mut new_paths = vec![];
                paths.iter().for_each(|(x, y)| {
                    [(-1, 0), (1, 0), (0, -1), (0, 1)]
                        .iter()
                        .for_each(|&(d_x, d_y)| {
                            let new_x = *x as i32 + d_x;
                            let new_y = *y as i32 + d_y;
                            if new_x >= 0 && new_y >= 0 {
                                let new_x = new_x as usize;
                                let new_y = new_y as usize;
                                if map.get(&(new_x, new_y)) == Some(&cur_num) {
                                    new_paths.push((new_x, new_y));
                                }
                            }
                        });
                });
                if unique {
                    new_paths.sort_unstable();
                    new_paths.dedup();
                }
                paths = new_paths;
                if paths.is_empty() {
                    break;
                }
            }
            count += paths.len();
        }

        count
    }
    pub fn first_part(&self, input: &str) -> usize {
        self.calc_num_paths(input, true)
    }

    pub fn second_part(&self, input: &str) -> usize {
        self.calc_num_paths(input, false)
    }
}

#[cfg(test)]
mod tests {
    use crate::day10::Day10;

    #[test]
    fn first_part_test() {
        assert_eq!(Day10.first_part(include_str!("day10_input_test.txt")), 36);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day10.first_part(include_str!("day10_input.txt")), 468);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day10.second_part(include_str!("day10_input_test.txt")), 81);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day10.second_part(include_str!("day10_input.txt")), 966);
    }
}
