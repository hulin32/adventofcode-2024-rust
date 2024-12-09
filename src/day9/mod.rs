pub struct Day9;

impl Day9 {
    fn filter_input(&self, input: &str) -> Vec<String> {
        let mut disk = vec![];
        input
            .chars()
            .enumerate()
            .for_each(|(i, c)| {
                // if i is odd, duplicate c i times, or duplicate . c times
                let times = c.to_string().parse::<i64>().unwrap();
                for _ in 0..times {
                    if i % 2 == 0 {
                        disk.push((i/2).to_string());
                    } else {
                        disk.push(".".to_string());
                    }
                }
            });
        disk
    }
    pub fn first_part(&self, input: &str) -> i64 {
        let disk_list =  self.filter_input(input);
        let mut end_idx = disk_list.len() - 1;
        let mut rearranged_disk = vec![];
        for (i, v) in disk_list.iter().enumerate() {
            if i > end_idx {
                break;
            }
            if v == "." {
                while disk_list[end_idx] == "." {
                    end_idx -= 1;
                }
                rearranged_disk.push(disk_list[end_idx].parse::<i64>().unwrap());
                end_idx -= 1;
            } else {
                rearranged_disk.push(v.parse::<i64>().unwrap());
            }
        }
        rearranged_disk.iter().enumerate().map(|(i, v)| {
            (i as i64)* v
        }).sum()
    }

    pub fn second_part(&self, input: &str) -> i64 {
        1
    }
}

#[cfg(test)]
mod tests {
    use crate::day9::Day9;

    #[test]
    fn first_part_test() {
        assert_eq!(Day9.first_part(include_str!("day9_input_test.txt")), 1928);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day9.first_part(include_str!("day9_input.txt")), 6334655979668);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day9.second_part(include_str!("day9_input_test.txt")), 34);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day9.second_part(include_str!("day9_input.txt")), 1308);
    }
}
