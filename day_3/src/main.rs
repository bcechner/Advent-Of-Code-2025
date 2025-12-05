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

        max_joltages.push(find_max_joltage(line, 12));
    }

    println!("{:?}", max_joltages);
    println!("{}", max_joltages.iter().sum::<i64>());
}

fn test() {
    assert!(find_max_joltage("987654321111111".to_string(), 12) == 987654321111);
    assert!(find_max_joltage("811111111111119".to_string(), 12) == 811111111119);
    assert!(find_max_joltage("234234234234278".to_string(), 12) == 434234234278);
    assert!(find_max_joltage("818181911112111".to_string(), 12) == 888911112111);
    
    println!("Test completed! Proceed.");
    println!();
}

fn find_max_joltage(bank: String, number_of_digits: i32) -> i64 {
    let bank_digits: Vec<i32> = bank.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let mut max_joltage_digits: Vec<i32> = Vec::new();

    let mut leftwards_searchable_index = 0;
    
    for current_digit in 0..number_of_digits {
        max_joltage_digits.push(0);

        println!("Evaluating digit {}", current_digit);

        for current_position in (leftwards_searchable_index..(bank_digits.len() - (number_of_digits - current_digit - 1) as usize) as i32).rev() {
            println!("Considering position {0}, with value {1}", current_position, bank_digits[current_position as usize]);

            if bank_digits[current_position as usize] >= max_joltage_digits[current_digit as usize] {
                println!("Found new highest {}", bank_digits[current_position as usize]);
                max_joltage_digits[current_digit as usize] = bank_digits[current_position as usize];

                leftwards_searchable_index = current_position + 1;
            }
        }
    }

    let max_joltage = max_joltage_digits.iter()
                             .map(|digit| digit.to_string())
                             .collect::<Vec<String>>()
                             .join("")
                             .parse()
                             .unwrap();

    println!("Max joltage: {max_joltage}");

    max_joltage
}