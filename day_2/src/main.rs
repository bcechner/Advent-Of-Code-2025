use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use claims::assert_ok;

fn main() {
    test_invalid_id_finder();

    let mut invalid_id_finder = InvalidIdFinder::new();

    match File::open("src/input.txt") {
        Ok(f) => {
            let reader = BufReader::new(f);

            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        l.split(",").for_each(|id_range| {
                            let id_range_parts: Vec<i64> = id_range.split("-")
                                                                   .map(|id_part| 
                                                                        {  
                                                                            return assert_ok!(id_part.parse::<i64>()) 
                                                                        })
                                                                    .collect();

                            invalid_id_finder.add_invalid_ids_in_range(id_range_parts[0], id_range_parts[1]);
                        });
                    }
                    Err(e) => panic!("Failed to read line: {e}")
                }
            }
        }
        Err(e) => panic!("Failed to read the input file: {e}")
    }

    println!("Final sum: {}", invalid_id_finder.sum_invalid_ids());
}

fn test_invalid_id_finder() {
    let mut invalid_id_finder = InvalidIdFinder::new();

    invalid_id_finder.add_invalid_ids_in_range(11, 22);
    invalid_id_finder.add_invalid_ids_in_range(95, 115);
    invalid_id_finder.add_invalid_ids_in_range(998, 1012);
    invalid_id_finder.add_invalid_ids_in_range(1188511880, 1188511890);
    invalid_id_finder.add_invalid_ids_in_range(222220, 222224);
    invalid_id_finder.add_invalid_ids_in_range(1698522, 1698528);
    invalid_id_finder.add_invalid_ids_in_range(446443, 446449);
    invalid_id_finder.add_invalid_ids_in_range(38593856, 38593862);

    let sum = invalid_id_finder.sum_invalid_ids();

    assert!(sum == 1227775554, "Sum was incorrect: {sum}");

    println!("Test complete! Proceed.");
    println!();
}

struct InvalidIdFinder {
    invalid_ids: Vec<i64>
}

impl InvalidIdFinder {
    fn new() -> InvalidIdFinder {
        InvalidIdFinder { invalid_ids: Vec::new() }
    }

    fn add_invalid_ids_in_range(&mut self, starting_id: i64, ending_id: i64) {
        for id in starting_id..=ending_id {
            let stringified_id = id.to_string();

            println!("Processing {stringified_id}");

            if stringified_id.len() % 2 != 0 {
                println!("Can't split, so skipping...");

                continue;
            }

            let first_part = &stringified_id[0..(&stringified_id.len()/2)];
            let second_part = &stringified_id[(&stringified_id.len()/2)..stringified_id.len()];

            println!("{first_part}");
            println!("{second_part}");

            if first_part == second_part {
                println!("That one was invalid!");

                self.invalid_ids.push(id)
            }
        }
    }

    fn sum_invalid_ids(&self) -> i64 {
        return self.invalid_ids.iter().sum();
    }
}