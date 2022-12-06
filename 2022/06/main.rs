use std::env;
use std::fs;

fn slice_repeated(s: &str) -> bool {
    for (i, c) in s.chars().enumerate() {
        if format!("{}{}",&s[..i] , &s[i + 1..]).contains(c) {
            return true
        }
    }
    return false
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Usage: {} <number>", args[0]);
        return;
    }

    let content = fs::read_to_string(&args[1]).unwrap();

    let pack_len = 14;
    let mut max = 0;
    for (i, c)  in content.chars().enumerate() {
        if i <= pack_len + max {
            if content[..i].contains(c) {
                max += 1;
            }
        } else {
            let slice = &content[i-pack_len+1..i+1];
            if !slice[..pack_len-1].contains(c) && !slice_repeated(&slice) {
                println!("found at index: {}", i + 1);
                return;
            } else {

            }
        }
    }
}

