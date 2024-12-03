use std::collections::BinaryHeap;

const INPUT: &str = include_str!("./input.txt");

fn main() {
    let input_list = INPUT.lines();

    let mut left_list = BinaryHeap::new();
    let mut right_list = BinaryHeap::new();

    for line in input_list {
        let mut line_numbers = line.split_whitespace();

        let left = line_numbers.next().map(|x| x.parse::<u32>());

        match left {
            Some(Ok(x)) => left_list.push(x),
            Some(Err(_)) => continue,
            None => continue,
        }

        let right = line_numbers.next().map(|x| x.parse::<u32>());

        match right {
            Some(Ok(x)) => right_list.push(x),
            Some(Err(_)) => continue,
            None => continue,
        }
    }

    let mut out_sum: u32 = 0;

    loop {
        let left = left_list.pop();
        let right = right_list.pop();

        if left > right {
            out_sum += left.unwrap() - right.unwrap();
        } else {
            out_sum += right.unwrap() - left.unwrap();
        }

        if left_list.is_empty() && right_list.is_empty() {
            break;
        }
    }

    println!("AOC 2024 Day 1 Part 1 Solution: {}", out_sum);
}
