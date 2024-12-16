use std::collections::HashMap;

pub struct Day12;

impl Day12 {
    fn calculate_price(&self, inputs: Vec<(char, Vec<(i32, i32)>)>) -> i64 {
        inputs
            .iter()
            .map(|(c, v)| {
                println!("{:?} {:?} {:?}", c, v.len(), v);
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
    pub fn first_part(&self, input: &str) -> i64 {
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

        self.calculate_price(data)
    }

    pub fn second_part(&self, input: &str) -> usize {
        1
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
        assert_eq!(Day12.second_part(include_str!("day12_input_test.txt")), 1206);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day12.second_part(include_str!("day12_input.txt")), 966);
    }
}
