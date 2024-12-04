use std::collections::HashMap;

pub struct Day4;

impl Day4 {
    pub fn first_part(&self, input: &str) -> i32 {
        let word = "XMAS";
        let mut list: Vec<(i32, i32, &str)> = vec![];
        let mut map: HashMap<(i32, i32), &str> = HashMap::new();
        input.lines().enumerate().for_each(|(x, line)| {
            line.split("").enumerate().for_each(|(y, char)| {
                if word.contains(char) {
                    list.push((x as i32, y as i32, char));
                    map.insert((x as i32, y as i32), char);
                }
            });
        });
        let mut sum = 0;
        // search from top down, and right to left
        list.iter().for_each(|(x, y, _)| {
            if map.get(&(*x, *y)) == Some(&"X") {
                // search down left
                if map.get(&(*x + 1, *y - 1)) == Some(&"M")
                    && map.get(&(*x + 2, *y - 2)) == Some(&"A")
                    && map.get(&(*x + 3, *y - 3)) == Some(&"S")
                {
                    sum += 1;
                }

                // search down
                if map.get(&(*x + 1, *y)) == Some(&"M")
                    && map.get(&(*x + 2, *y)) == Some(&"A")
                    && map.get(&(*x + 3, *y)) == Some(&"S")
                {
                    sum += 1;
                }

                // search down right
                if map.get(&(*x + 1, *y + 1)) == Some(&"M")
                    && map.get(&(*x + 2, *y + 2)) == Some(&"A")
                    && map.get(&(*x + 3, *y + 3)) == Some(&"S")
                {
                    sum += 1;
                }

                // search right
                if map.get(&(*x, *y + 1)) == Some(&"M")
                    && map.get(&(*x, *y + 2)) == Some(&"A")
                    && map.get(&(*x, *y + 3)) == Some(&"S")
                {
                    sum += 1;
                }
            }
            if map.get(&(*x, *y)) == Some(&"S") {
                // search down left
                if map.get(&(*x + 1, *y - 1)) == Some(&"A")
                    && map.get(&(*x + 2, *y - 2)) == Some(&"M")
                    && map.get(&(*x + 3, *y - 3)) == Some(&"X")
                {
                    sum += 1;
                }

                // search down
                if map.get(&(*x + 1, *y)) == Some(&"A")
                    && map.get(&(*x + 2, *y)) == Some(&"M")
                    && map.get(&(*x + 3, *y)) == Some(&"X")
                {
                    sum += 1;
                }

                // search down right
                if map.get(&(*x + 1, *y + 1)) == Some(&"A")
                    && map.get(&(*x + 2, *y + 2)) == Some(&"M")
                    && map.get(&(*x + 3, *y + 3)) == Some(&"X")
                {
                    sum += 1;
                }

                // search right
                if map.get(&(*x, *y + 1)) == Some(&"A")
                    && map.get(&(*x, *y + 2)) == Some(&"M")
                    && map.get(&(*x, *y + 3)) == Some(&"X")
                {
                    sum += 1;
                }
            }
        });
        sum
    }

    pub fn second_part(&self, input: &str) -> i32 {
        let word = "MAS";
        let mut list: Vec<(i32, i32, &str)> = vec![];
        let mut map: HashMap<(i32, i32), &str> = HashMap::new();
        input.lines().enumerate().for_each(|(x, line)| {
            line.split("").enumerate().for_each(|(y, char)| {
                if word.contains(char) {
                    list.push((x as i32, y as i32, char));
                    map.insert((x as i32, y as i32), char);
                }
            });
        });
        let mut sum = 0;
        // search from top down, and right to left
        list.iter().for_each(|(x, y, _)| {
            if map.get(&(*x, *y)) == Some(&"A") {
                // search down left
                let mut left = false;
                if (map.get(&(*x - 1, *y - 1)) == Some(&"M")
                    && map.get(&(*x + 1, *y + 1)) == Some(&"S"))
                    || (map.get(&(*x - 1, *y - 1)) == Some(&"S")
                        && map.get(&(*x + 1, *y + 1)) == Some(&"M"))
                {
                    left = true;
                }
                let mut right = false;
                // search down
                if (map.get(&(*x + 1, *y - 1)) == Some(&"M")
                    && map.get(&(*x - 1, *y + 1)) == Some(&"S"))
                    || (map.get(&(*x + 1, *y - 1)) == Some(&"S")
                        && map.get(&(*x - 1, *y + 1)) == Some(&"M"))
                {
                    right = true;
                }
                if left && right {
                    sum += 1;
                }
            }
        });
        sum
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::Day4;

    #[test]
    fn first_part_test() {
        assert_eq!(Day4.first_part(include_str!("day4_input_test1.txt")), 18);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day4.first_part(include_str!("day4_input.txt")), 2534);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day4.second_part(include_str!("day4_input_test2.txt")), 9);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day4.second_part(include_str!("day4_input.txt")), 1866);
    }
}
