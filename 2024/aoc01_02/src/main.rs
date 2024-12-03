use std::collections::{HashMap, HashSet};

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let input_list = INPUT.lines();

    let mut map_appearances: HashMap<u32, u32> = HashMap::new();
    let mut unique_keys: HashSet<u32> = HashSet::new();

    for line in input_list {
        let mut line_numbers = line.split_whitespace();

        let left = line_numbers
            .next()
            .map(|x| x.parse::<u32>().unwrap())
            .unwrap();

        unique_keys.insert(left);

        let right = line_numbers
            .next()
            .map(|x| x.parse::<u32>().unwrap())
            .unwrap();

        if !map_appearances.contains_key(&right) {
            map_appearances.insert(right, 1);
        } else {
            let value = map_appearances.get(&right).unwrap();
            map_appearances.insert(right, value + 1);
        }
    }

    let mut out_sum: u32 = 0;

    for key in unique_keys {
        let value = map_appearances.get(&key);
        if value == None {
            continue;
        }
        out_sum += key * value.unwrap();
    }

    println!("AOC 2024 Day 1 Part 2 Solution: {}", out_sum);
}
