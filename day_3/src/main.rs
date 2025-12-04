use std::fs;

fn main() {
    let data = fs::read_to_string("joltage.txt").unwrap();
    let banks = data.lines();
    let mut total_output = 0;

    for (i, bank) in banks.enumerate() {
        println!("-------- NEW BANK #{}--------", i);
        //
        // Set Init Values
        let mut first_max_num: u32 = 0;
        let mut first_i = 0;
        let mut second_max_num: u32 = 0;
        let mut second_i = 0;

        // First Highest Number Check
        for (i, c) in bank.chars().enumerate() {
            let current_num = c.to_digit(10).unwrap();
            if current_num > first_max_num && i != bank.len() - 1 {
                first_max_num = current_num;
                first_i = i;
            }
        }
        // Second Highest Number Check
        for (i, c) in bank.chars().enumerate() {
            let current_num = c.to_digit(10).unwrap();
            if current_num > second_max_num && i > first_i {
                second_max_num = current_num;
                second_i = i;
            }
        }

        let max_joltage = (first_max_num * 10) + second_max_num;
        total_output += max_joltage;

        // Terminal Output
        println!(
            "First Max Num: {}, at Index: {}\nSecond Max Num: {}, at Index {}",
            first_max_num, first_i, second_max_num, second_i,
        );
        println!("Total Batteries: {}", bank.len());
        println!("Bank Max Joltage: {}", max_joltage);

        //
        //
    }
    println!("Total Output: {}", total_output);
}
