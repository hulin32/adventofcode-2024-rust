use std::collections::{HashMap, HashSet};

pub struct Day12;

impl Day12 {
    pub fn filter_data(&self, input: &str) -> Vec<(char, Vec<(i32, i32)>)> {
        let mut data: Vec<(char, Vec<(i32, i32)>)> = vec![];
        let mut points = HashMap::new();
        let mut visited = HashMap::new();
        let mut inputs = vec![];
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                points.insert((x as i32, y as i32), c);
                visited.insert((x as i32, y as i32), false);
                inputs.push((x as i32, y as i32));
            });
        });

        inputs.iter().for_each(|p| {
            if !visited.get(p).unwrap() {
                let c = points.get(p).unwrap();
                let mut area = vec![];
                area.push(*p);
                let mut current_inputs = vec![*p];
                loop {
                    let mut new_inputs = vec![];
                    current_inputs.iter().for_each(|pos| {
                        // check p's left, right, top, bottom
                        [
                            (pos.0 - 1, pos.1),
                            (pos.0 + 1, pos.1),
                            (pos.0, pos.1 - 1),
                            (pos.0, pos.1 + 1),
                        ]
                        .iter()
                        .for_each(|closed_pos| {
                            if points.contains_key(closed_pos)
                                && points.get(closed_pos).unwrap() == c
                                && visited.contains_key(closed_pos)
                                && !visited.get(closed_pos).unwrap()
                            {
                                new_inputs.push(*closed_pos);
                                visited.insert(*closed_pos, true);
                                if !area.contains(closed_pos) {
                                    area.push(*closed_pos);
                                }
                            }
                        });
                    });
                    if new_inputs.is_empty() {
                        break;
                    }
                    current_inputs = new_inputs;
                }
                data.push((*c, area));
            }
        });

        data
    }

    fn first_part(&self, inputs: &str) -> i64 {
        let data = self.filter_data(inputs);
        data
            .iter()
            .map(|(c, v)| {
                let perimeter = v
                    .iter()
                    .map(|(x, y)| {
                        // check left, top, right, bottom
                        let mut count = 0;
                        if !v.contains(&(x - 1, *y)) {
                            count += 1;
                        }
                        if !v.contains(&(*x, y - 1)) {
                            count += 1;
                        }
                        if !v.contains(&(x + 1, *y)) {
                            count += 1;
                        }
                        if !v.contains(&(*x, y + 1)) {
                            count += 1;
                        }
                        count
                    })
                    .sum::<i64>();
                perimeter * (v.len() as i64)
            })
            .sum()
    }


    fn update_edges(&self, edges: &mut HashMap<i32, HashSet<i32>>, key: i32, val: i32) {
        edges.entry(key).and_modify(|v| {
            v.insert(val);
        }).or_insert_with(|| HashSet::from([val]));
        edges.get_mut(&key).unwrap();
    }
    pub fn second_part(&self, input: &str) -> i32 {
        let data = self.filter_data(input);
        data
            .iter()
            .map(|(c, v)| {
                let mut x_edges = HashMap::new();
                let mut y_edges = HashMap::new();
                v
                    .iter()
                    .for_each(|(x, y)| {
                        // check left, top, right, bottom
                        if !v.contains(&(x - 1, *y)) {
                            self.update_edges(&mut y_edges, *x*10 - 4, *y*10);
                        }
                        if !v.contains(&(*x, y - 1)) {
                            self.update_edges(&mut x_edges, *y*10 - 4, *x*10);
                        }
                        if !v.contains(&(x + 1, *y)) {
                            self.update_edges(&mut y_edges, *x*10 + 5, *y*10);
                        }
                        if !v.contains(&(*x, y + 1)) {
                            self.update_edges(&mut x_edges, *y*10 + 5, *x*10);
                        }
                    });
                let mut edge_count = 0;
                // if (*c == 'B') {
                //     println!("{:?}", x_edges);
                //     println!("{:?}", y_edges);
                // }
                x_edges.values().for_each(|xs| {
                    let mut vals: Vec<&i32> = xs.iter().collect();
                    vals.sort();
                    let mut current = vals[0] - 10;
                    edge_count += 1;
                    for x in vals {
                        if x - 10 != current {
                            edge_count += 1;
                        }
                        current = *x;
                    }
                });
                y_edges.values().for_each(|ys| {
                    let mut vals: Vec<&i32> = ys.iter().collect();
                    vals.sort();
                    let mut current = vals[0] - 10;
                    edge_count += 1;
                    for x in vals {
                        if x - 10 != current {
                            edge_count += 1;
                        }
                        current = *x;
                    }
                });

                println!("{:?}, {:?}, {:?}", c, edge_count, v.len());

                edge_count * v.len() as i32
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day12::Day12;

    #[test]
    fn first_part_test() {
        assert_eq!(Day12.first_part(include_str!("day12_input_test.txt")), 1930);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day12.first_part(include_str!("day12_input.txt")), 1304764);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day12.second_part(include_str!("day12_input_test.txt")), 368);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day12.second_part(include_str!("day12_input.txt")), 966);
    }
}
