use core::fmt;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    test_counter();

    match File::open("src/input.txt") {
        Ok(f) => {
            let reader = BufReader::new(f);

            let mut dial_counter = DialCounter::new();

            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        dial_counter.rotate(l);
                    }
                    Err(e) => panic!("Failed to read line: {e}")
                }
            }
        }
        Err(e) => panic!("Failed to read the input file: {e}")
    }
}

fn test_counter() {
    let mut dial_counter = DialCounter::new();

    println!("Running test: ");

    dial_counter.rotate("L68".to_string());
    dial_counter.rotate("L30".to_string());
    dial_counter.rotate("R48".to_string());
    dial_counter.rotate("L5".to_string());
    dial_counter.rotate("R60".to_string());
    dial_counter.rotate("L55".to_string());
    dial_counter.rotate("L1".to_string());
    dial_counter.rotate("L99".to_string());
    dial_counter.rotate("R14".to_string());
    dial_counter.rotate("L82".to_string());

    assert!(dial_counter.current_position == 32, "Dial counter doesn't rotate correctly");
    assert!(dial_counter.number_of_post_rotation_zeroes == 3, "Dial counter doesn't count post rotation zeroes correctly");
    assert!(dial_counter.number_of_mid_rotation_zeroes == 6, "Dial counter doesn't count mid rotation zeroes correctly");

    println!("Test completed succesfully. Proceed.");
    println!("");
}

struct DialCounter {
    current_position: i32,
    number_of_post_rotation_zeroes: i32,
    number_of_mid_rotation_zeroes: i32
}

impl DialCounter {
    fn new() -> DialCounter {
        DialCounter { 
            current_position: 50,
            number_of_post_rotation_zeroes: 0,
            number_of_mid_rotation_zeroes: 0
        }
    }

    fn rotate(&mut self, code: String) {
        let op_code = &code[0..1];
        let rotation_amount: i32 = *&code[1..code.len()].parse::<i32>().expect("Failed to read operation code");

        println!("Operation: {0}, rotation amount: {1}", op_code, rotation_amount);

        match op_code {
            "R" => {
                for _i in 0..rotation_amount {
                    self.current_position += 1;

                    if self.current_position > 99 {
                        self.current_position = 0;
                    }

                    if self.current_position == 0 {
                        self.number_of_mid_rotation_zeroes += 1;
                    }
                }
            }
            "L" => {
                for _i in 0..rotation_amount {
                    self.current_position -= 1;

                    if self.current_position < 0 {
                        self.current_position = 99;
                    }

                    if self.current_position == 0 {
                        self.number_of_mid_rotation_zeroes += 1;
                    }
                }
            }
            _ => {
                println!("Invalid operation code, skipping");
                return;
            }
        }

        if self.current_position == 0 {
            self.number_of_post_rotation_zeroes += 1;        
        }

        println!("{}", self);
    }
}

impl fmt::Display for DialCounter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,
               "Current position: {0}, with {1} post zeroes, and {2} mid zeroes",
               self.current_position,
               self.number_of_post_rotation_zeroes,
               self.number_of_mid_rotation_zeroes
            )
    }
}