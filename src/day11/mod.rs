use std::collections::HashMap;

pub struct Day11 {
    caches: HashMap<(i64, usize), i64>,
}

impl Day11 {
    fn count(&mut self, stone: i64, steps: usize) -> i64 {
        if let Some(count) = self.caches.get(&(stone, steps)) {
            return *count;
        }
        if steps == 0 {
            return 1;
        }
        if stone == 0 {
            let count = self.count(1, steps - 1);
            self.caches.insert((1, steps - 1), count);
            return count;
        }
        if stone.to_string().len() % 2 == 0 {
            let mid = stone.to_string().len() / 2;
            let count_left =
                self.count(stone.to_string()[..mid].parse::<i64>().unwrap(), steps - 1);
            self.caches.insert(
                (stone.to_string()[..mid].parse::<i64>().unwrap(), steps - 1),
                count_left,
            );
            let count_right =
                self.count(stone.to_string()[mid..].parse::<i64>().unwrap(), steps - 1);
            self.caches.insert(
                (stone.to_string()[mid..].parse::<i64>().unwrap(), steps - 1),
                count_right,
            );
            return count_left + count_right;
        }
        let count = self.count(stone * 2024, steps - 1);
        self.caches.insert((stone * 2024, steps - 1), count);
        count
    }

    pub fn first_part(&mut self, input: &str, times: usize) -> i64 {
        let inputs: Vec<i64> = input
            .split(" ")
            .filter(|s| !s.is_empty())
            .map(|c| c.parse::<i64>().unwrap())
            .collect();
        let mut count = 0;
        inputs.iter().for_each(|&x| {
            count += self.count(x, times);
        });

        count
    }
}

#[cfg(test)]
mod tests {
    use crate::day11::Day11;
    use std::collections::HashMap;

    #[test]
    fn first_part_test() {
        let mut day11 = Day11 {
            caches: HashMap::new(),
        };
        assert_eq!(
            day11.first_part(include_str!("day11_input_test.txt"), 25),
            55312
        );
    }

    #[test]
    fn first_part() {
        let mut day11 = Day11 {
            caches: HashMap::new(),
        };
        assert_eq!(
            day11.first_part(include_str!("day11_input.txt"), 25),
            183620
        );
    }

    #[test]
    fn second_part() {
        let mut day11 = Day11 {
            caches: HashMap::new(),
        };
        assert_eq!(
            day11.first_part(include_str!("day11_input.txt"), 75),
            220377651399268
        );
    }
}
