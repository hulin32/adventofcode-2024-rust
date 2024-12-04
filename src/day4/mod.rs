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
mod tests_version1 {
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

pub struct Day4Version2;

impl Day4Version2 {
    fn get_data<'a>(&self, input: &'a str, incuded_chars: &str) -> HashMap<(i32, i32), &'a str> {
        let mut map: HashMap<(i32, i32), &str> = HashMap::new();
        input.lines().enumerate().for_each(|(x, line)| {
            line.split("").enumerate().for_each(|(y, char)| {
                if incuded_chars.contains(char) {
                    map.insert((x as i32, y as i32), char);
                }
            });
        });
        map
    }

    fn if_all_match<'a>(
        &self,
        map: &HashMap<(i32, i32), &'a str>,
        x: i32,
        y: i32,
        cases: &[(i32, i32)],
        word: &[&str],
    ) -> bool {
        cases
            .iter()
            .zip(word.iter())
            .all(|((i, j), char)| map.get(&(x + i, y + j)) == Some(char))
    }

    pub fn first_part(&self, input: &str) -> i32 {
        let map = self.get_data(input, "XMAS");
        let mut sum = 0;
        let cases = [
            // search down left
            [(0, 0), (1, -1), (2, -2), (3, -3)],
            // search down
            [(0, 0), (1, 0), (2, 0), (3, 0)],
            // search down right
            [(0, 0), (1, 1), (2, 2), (3, 3)],
            // search right
            [(0, 0), (0, 1), (0, 2), (0, 3)],
        ];
        let words = [["X", "M", "A", "S"], ["S", "A", "M", "X"]];
        // search from top down, and right to left
        map.iter().for_each(|((x, y), char)| {
            // filter out all cases which is not start from X/S
            if char == &"X" || char == &"S" {
                words.iter().for_each(|word| {
                    cases.iter().for_each(|case| {
                        if self.if_all_match(&map, *x, *y, case, word) {
                            sum += 1;
                        }
                    });
                });
            }
        });
        sum
    }

    pub fn second_part(&self, input: &str) -> i32 {
        let map = self.get_data(input, "MAS");
        let mut sum = 0;
        let cases = [[(0, 0), (-1, -1), (1, 1)], [(0, 0), (1, -1), (-1, 1)]];
        let words = [["A", "M", "S"], ["A", "S", "M"]];
        // search from top down, and right to left
        map.iter().for_each(|((x, y), char)| {
            // filter out all cases which is not  A
            if char == &"A"
                && cases.iter().all(|case| {
                    words
                        .iter()
                        .any(|word| self.if_all_match(&map, *x, *y, case, word))
                })
            {
                sum += 1;
            }
        });
        sum
    }
}

#[cfg(test)]
mod tests_version2 {
    use crate::day4::Day4Version2;

    #[test]
    fn first_part_test() {
        assert_eq!(
            Day4Version2.first_part(include_str!("day4_input_test1.txt")),
            18
        );
    }

    #[test]
    fn first_part() {
        assert_eq!(
            Day4Version2.first_part(include_str!("day4_input.txt")),
            2534
        );
    }

    #[test]
    fn second_part_test() {
        assert_eq!(
            Day4Version2.second_part(include_str!("day4_input_test2.txt")),
            9
        );
    }

    #[test]
    fn second_part() {
        assert_eq!(
            Day4Version2.second_part(include_str!("day4_input.txt")),
            1866
        );
    }
}
