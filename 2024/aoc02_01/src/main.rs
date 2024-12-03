const INPUT: &str = include_str!("./input.txt");

fn main() {
    let mut safe_reports: u32 = 0;

    for line in INPUT.lines() {
        let mut data = line.split_whitespace();

        let mut previous_data_point = data.next().unwrap().parse::<u32>().unwrap();
        let mut current_data_point = data.next().unwrap().parse::<u32>().unwrap();

        let first_dif = current_data_point.abs_diff(previous_data_point);
        if first_dif < 1 || first_dif > 3 {
            continue;
        }

        let direction = if current_data_point > previous_data_point {
            1
        } else {
            -1
        };

        let mut is_safe = true;
        loop {
            previous_data_point = current_data_point;
            let next_val = data.next();
            if next_val.is_none() {
                break;
            }
            current_data_point = next_val.unwrap().parse::<u32>().unwrap();
            let diff = current_data_point.abs_diff(previous_data_point);

            let new_direction = if current_data_point > previous_data_point {
                1
            } else {
                -1
            };

            if new_direction != direction {
                is_safe = false;
                break;
            }

            if diff < 1 || diff > 3 {
                is_safe = false;
                break;
            }
        }

        if !is_safe {
            continue;
        }

        safe_reports += 1;
    }

    println!("AOC 2024 Day 2 Part 1 Solution: {}", safe_reports);
}
