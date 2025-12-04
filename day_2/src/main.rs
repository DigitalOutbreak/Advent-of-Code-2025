use day_2::is_new_repeated_pattern;
use std::fs;

fn main() {
    let data = fs::read_to_string("puzzle_input.txt").unwrap();

    let ranges = data.split(",");
    let mut added_invalid_ids = 0;
    // goes through the ranges
    for range in ranges {
        let range = range.trim();
        let (start_str, end_str) = range.split_once("-").unwrap();
        let start_num = start_str.parse::<u64>().unwrap();
        let end_num = end_str.parse::<u64>().unwrap();
        //iterates through the range and parses to string so we can find patters

        for num in start_num..=end_num {
            // let has_pattern = is_repeated_pattern(num);
            let has_new_pattern = is_new_repeated_pattern(num);
            if has_new_pattern {
                println!("{} is INVALID", num);
                added_invalid_ids += num;
            }
        }
    }
    println!("Total of invalid ID's {}", added_invalid_ids);
}
