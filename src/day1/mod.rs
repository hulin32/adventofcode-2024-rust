use std::collections::HashMap;

pub struct Day1;

impl Day1 {
    pub fn first_part(input: &str) -> i32 {
        let mut f_q = vec![];
        let mut s_q = vec![];
        input.lines().for_each(|l| {
            let stars: Vec<i32> = l.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            f_q.push(stars.first().unwrap().to_owned());
            s_q.push(stars.get(1).unwrap().to_owned());
        });
        f_q.sort();
        s_q.sort();
        f_q.iter().zip(s_q.iter()).map(|(f, s)| (f - s).abs()).sum()
    }

    pub fn second_part(input: &str) -> i32 {
        let mut f_q = vec![];
        let mut map = HashMap::new();
        input.lines().for_each(|l| {
            let stars: Vec<i32> = l.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
            f_q.push(stars.first().unwrap().to_owned());
            let t = stars.get(1).unwrap().to_owned();
            if map.contains_key(&t) {
                map.insert(t, map.get(&t).unwrap() + 1);
            } else {
                map.insert(t, 1);
            }
        });
        f_q.iter().map(|f| {
            if map.contains_key(f) {
                f * map.get(f).unwrap()
            } else {
                0
            }
        }).sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::day1::Day1;

    #[test]
    fn first_part_test() {
        assert_eq!(Day1::first_part(include_str!("day1_input_test1.txt")), 11);
    }

    #[test]
    fn first_part() {
        assert_eq!(Day1::first_part(include_str!("day1_input.txt")), 1590491);
    }

    #[test]
    fn second_part_test() {
        assert_eq!(Day1::second_part(include_str!("day1_input_test2.txt")), 31);
    }

    #[test]
    fn second_part() {
        assert_eq!(Day1::second_part(include_str!("day1_input.txt")), 22588371);
    }
}