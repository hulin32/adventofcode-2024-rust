pub struct Day9;

impl Day9 {
    fn first_part_filter_input(&self, input: &str) -> Vec<String> {
        let mut disk = vec![];
        input.chars().enumerate().for_each(|(i, c)| {
            // if i is odd, duplicate c i times, or duplicate . c times
            let times = c.to_string().parse::<i64>().unwrap();
            for _ in 0..times {
                if i % 2 == 0 {
                    disk.push((i / 2).to_string());
                } else {
                    disk.push(".".to_string());
                }
            }
        });
        disk
    }
    pub fn first_part(&self, input: &str) -> i64 {
        let disk_list = self.first_part_filter_input(input);
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
        rearranged_disk
            .iter()
            .enumerate()
            .map(|(i, v)| (i as i64) * v)
            .sum()
    }

    fn second_part_filter_input(&self, input: &str) -> Vec<String> {
        let mut disk = vec![];
        let mut empty_area: Vec<(usize, i8)> = vec![];
        let mut file_area: Vec<(usize, i8)> = vec![];
        let mut max_size: i8 = 0;
        input.chars().enumerate().for_each(|(i, c)| {
            // if i is odd, duplicate c i times, or duplicate . c times
            let size = c.to_string().parse::<i8>().unwrap();
            if i % 2 == 0 {
                file_area.push((disk.len(), size));
            } else {
                max_size = max_size.max(size);
                empty_area.push((disk.len(), size));
            }
            for _ in 0..size {
                if i % 2 == 0 {
                    disk.push((i / 2).to_string());
                } else {
                    disk.push(".".to_string());
                }
            }
        });
        for (i, size) in file_area.iter().rev() {
            for (empty_idx, (empty_pos, empty_size)) in empty_area.clone().iter().enumerate() {
                if empty_pos < i && size <= empty_size {
                    // fill empty space
                    for j in *empty_pos..(*empty_pos + (*size as usize)) {
                        disk[j] = disk[*i].clone().to_string();
                    }
                    // empty orginal space
                    for j in *i..(*i + (*size as usize)) {
                        disk[j] = ".".to_string();
                    }
                    // empty original area
                    let left_start_idex = *empty_pos + (*size as usize);
                    empty_area[empty_idx] = (left_start_idex, empty_size - size);
                    break;
                }
            }
        }
        disk
    }

    pub fn second_part(&self, input: &str) -> i64 {
        let disk_list = self.second_part_filter_input(input);
        disk_list
            .iter()
            .enumerate()
            .map(|(i, v)| (i as i64) * v.parse::<i64>().unwrap_or_default())
            .sum()
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
        assert_eq!(
            Day9.first_part(include_str!("day9_input.txt")),
            6334655979668
        );
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day9.second_part(include_str!("day9_input_test.txt")), 2858);
    }

    #[test]
    fn second_part() {
        assert_eq!(
            Day9.second_part(include_str!("day9_input.txt")),
            6349492251099
        );
    }
}
