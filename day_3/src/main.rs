use std::fs;

fn main() {
    let data = fs::read_to_string("joltage.txt").unwrap();
    let banks = data.lines();
    let mut total_output: u64 = 0;

    // how many digits we must pick per bank (12 for part 2, 2 for part 1)
    const K: usize = 12;

    for (bank_idx, bank) in banks.enumerate() {
        println!("-------- BANK #{} --------", bank_idx + 1);

        let mut chosen_digits: Vec<u32> = Vec::with_capacity(K);
        let mut start_index: usize = 0; // where we’re allowed to start searching

        // choose K digits, one by one
        for chosen_so_far in 0..K {
            let remaining_after_this = K - chosen_so_far - 1;
            let (idx, digit) = pick_next_digit(bank, start_index, remaining_after_this);
            chosen_digits.push(digit);
            start_index = idx + 1; // next picks must be to the right
        }

        // turn the chosen digits into a number
        let mut bank_joltage: u64 = 0;
        for d in &chosen_digits {
            bank_joltage = bank_joltage * 10 + (*d as u64);
        }

        total_output += bank_joltage;

        println!("Bank:          {}", bank);
        println!("Chosen digits: {:?}", chosen_digits);
        println!("Bank joltage:  {}", bank_joltage);
        println!();
    }

    println!("Total Output: {}", total_output);
}

/// Pick the next best digit:
/// - `start_index`: we can only pick at or after this index
/// - `remaining_after`: how many digits we still need *after* this one
fn pick_next_digit(bank: &str, start_index: usize, remaining_after: usize) -> (usize, u32) {
    let len = bank.len();

    // Farthest index we’re allowed to pick from:
    // if we pick at i, we need `remaining_after` chars *after* i
    let max_index = len - remaining_after - 1;

    let mut best_digit: u32 = 0;
    let mut best_index: usize = start_index;

    // look only in [start_index ..= max_index]
    for (i, c) in bank
        .chars()
        .enumerate()
        .skip(start_index)
        .take(max_index - start_index + 1)
    {
        let current_num = c.to_digit(10).unwrap();
        if current_num > best_digit {
            best_digit = current_num;
            best_index = i;
        }
    }

    (best_index, best_digit)
}

// Gave up and USED AI for this one, got close though before giving up.
//
// Going back to studying Rust Syntex and Algo's. Will come back and try again in the future.
