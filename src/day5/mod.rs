use std::collections::{HashMap, HashSet};

pub struct Day5;

impl Day5 {
    fn calculate(
        &self,
        input: &str,
        callback: fn(update: &Vec<&str>, page_order: &HashMap<&str, Vec<&str>>) -> i32,
    ) -> i32 {
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
            .map(|update| callback(update, &page_order))
            .sum()
    }

    fn is_all_good(update: &[&str], page_order: &HashMap<&str, Vec<&str>>) -> bool {
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
        all_good
    }
    pub fn first_part(&self, input: &str) -> i32 {
        self.calculate(input, |update, page_order| {
            let all_good = Day5::is_all_good(update, page_order);
            if all_good {
                update[(update.len() - 1) / 2].parse::<i32>().unwrap()
            } else {
                0
            }
        })
    }

    // this fn is generated by claude.ai :D
    fn find_longest_chain(edges: &Vec<(i32, i32)>) -> Vec<i32> {
        // Create adjacency list representation
        let mut graph: HashMap<i32, Vec<i32>> = HashMap::new();
        let mut all_nodes: HashSet<i32> = HashSet::new();

        // Build the graph
        for &(parent, child) in edges {
            graph.entry(parent)
                .or_insert_with(Vec::new)
                .push(child);
            all_nodes.insert(parent);
            all_nodes.insert(child);
        }

        // Find all root nodes (nodes with no parents)
        let root_nodes: Vec<i32> = all_nodes
            .iter()
            .filter(|&&node| {
                !edges.iter().any(|&(_, child)| child == node)
            })
            .cloned()
            .collect();

        // DFS function to find longest path
        fn dfs(
            node: i32,
            graph: &HashMap<i32, Vec<i32>>,
            visited: &mut HashSet<i32>,
            current_path: &mut Vec<i32>,
            longest_path: &mut Vec<i32>
        ) {
            visited.insert(node);
            current_path.push(node);

            if current_path.len() > longest_path.len() {
                longest_path.clear();
                longest_path.extend(current_path.iter());
            }

            if let Some(children) = graph.get(&node) {
                for &child in children {
                    if !visited.contains(&child) {
                        dfs(child, graph, visited, current_path, longest_path);
                    }
                }
            }

            current_path.pop();
            visited.remove(&node);
        }

        // Try each root node as starting point
        let mut longest_path = Vec::new();
        let mut current_path = Vec::new();
        let mut visited = HashSet::new();

        for &root in &root_nodes {
            dfs(root, &graph, &mut visited, &mut current_path, &mut longest_path);
        }

        longest_path
    }

    // its directed graph
    pub fn second_part(&self, input: &str) -> i32 {
        self.calculate(input, |update, page_order| {
            let all_good = Day5::is_all_good(update, page_order);
            if !all_good {
                // filter out all valid pairs
                let mut valid_pairs = vec![];
                for i in 0..update.len() {
                    let current_page = update[i];
                    if page_order.contains_key(&current_page) {
                        let order = page_order.get(&current_page).unwrap();
                        for j in 0..update.len() {
                            if i != j {
                                let other_page = update[j];
                                if order.contains(&other_page) {
                                    valid_pairs
                                        .push((current_page.parse::<i32>().unwrap(), other_page.parse::<i32>().unwrap()));
                                }
                            }
                        }
                    }
                }

                let result = Day5::find_longest_chain(&valid_pairs);
                result[(result.len() - 1)/2]
            } else {
                0
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::day5::Day5;

    #[test]
    fn first_part_test() {
        assert_eq!(Day5.first_part(include_str!("day5_input_test.txt")), 143);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day5.first_part(include_str!("day5_input.txt")), 4578);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day5.second_part(include_str!("day5_input_test.txt")), 123);
    }

    #[test]
    fn second_part() {
        // TODO slow
        assert_eq!(Day5.second_part(include_str!("day5_input.txt")), 6179);
    }
}
