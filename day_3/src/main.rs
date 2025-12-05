use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use claims::assert_ok;

fn main() {
    test();

    let file = assert_ok!(File::open("src/input.txt")); 
    let reader = BufReader::new(file);

    let mut max_joltages:Vec<i64> = Vec::new();

    for line_result in reader.lines() {
        let line = assert_ok!(line_result);

        max_joltages.push(find_max_joltage(line));
    }

    println!("{:?}", max_joltages);
    println!("{}", max_joltages.iter().sum::<i64>());
}

fn test() {
    assert!(find_max_joltage("987654321111111".to_string()) == 98);
    assert!(find_max_joltage("811111111111119".to_string()) == 89);
    assert!(find_max_joltage("234234234234278".to_string()) == 78);
    assert!(find_max_joltage("818181911112111".to_string()) == 92);
    
    println!("Test completed! Proceed.");
    println!();
}

fn find_max_joltage(bank: String) -> i64 {
    let mut current_highest_joltage = 0;

    for i in 0..bank.len() {
        let candidate_first_digit = &bank[i..i+1];

        for j in i+1..bank.len() {
            let candidate_second_digit = &bank[j..j+1];

            let candidate_joltage = assert_ok!((candidate_first_digit.to_string() + candidate_second_digit).parse::<i64>());

            if candidate_joltage > current_highest_joltage {
                current_highest_joltage = candidate_joltage;
            }
        }
    }

    return current_highest_joltage;
}
