use std::fs;

fn main() {
    let data = fs::read_to_string("joltage.txt").unwrap();
    let banks = data.lines();

    for bank in banks {
        println!("{}", bank);
    }
}
