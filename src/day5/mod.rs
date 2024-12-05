use std::collections::HashMap;

pub struct Day5;

impl Day5 {
    pub fn first_part(&self, input: &str) -> i32 {
        let mut page_order: HashMap<&str, Vec<&str>> = HashMap::new();
        let mut updates = vec![];
        input.lines().for_each(|line| {
            if line.contains("|") {
                let orders = line.split("|").collect::<Vec<&str>>();
                if page_order.contains_key(orders[0]) {
                    let order = page_order.get_mut(orders[0]).unwrap();
                    order.push(orders[1]);
                } else {
                    page_order.insert(orders[0], vec![orders[1]]);
                }
            } else if !line.is_empty() {
                let nums: Vec<&str> = line.split(",").collect();
                updates.push(nums);
            }
        });

        updates
            .iter()
            .map(|update| {
                let mut all_good = true;
                for i in 0..update.len() - 1 {
                    let current_page = update[i];
                    let next_page = update[i + 1];
                    if page_order.contains_key(&current_page) {
                        let order = page_order.get(&current_page).unwrap();
                        if !order.contains(&next_page) {
                            all_good = false;
                            break;
                        }
                    } else {
                        all_good = false;
                        break;
                    }
                }
                if all_good {
                    update[(update.len() - 1) / 2].parse::<i32>().unwrap()
                } else {
                    0
                }
            })
            .sum()
    }

    pub fn second_part(&self, input: &str) -> i32 {
        2
    }
}

#[cfg(test)]
mod tests_version1 {
    use crate::day5::Day5;

    #[test]
    fn first_part_test() {
        assert_eq!(Day5.first_part(include_str!("day5_input_test1.txt")), 143);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day5.first_part(include_str!("day5_input.txt")), 4578);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day5.second_part(include_str!("day5_input_test2.txt")), 9);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day5.second_part(include_str!("day5_input.txt")), 1866);
    }
}
