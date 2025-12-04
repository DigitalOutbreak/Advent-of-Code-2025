use std::fs;

pub fn new_method() {
    let data = fs::read_to_string("rotations.txt").unwrap();

    let instructions = data.lines();
    let mut dial: i32 = 50;
    let mut password = 0;

    for instruction in instructions {
        // For each instruction collect the direction L or R
        let direction = instruction.chars().next().unwrap();

        // For each instruction collect step amount
        let step_amt = instruction[1..].parse::<i32>().unwrap();
        // For each instruction if L subtract amt if R add amt to/from dial
        match direction {
            'L' => {
                for _ in 0..step_amt {
                    dial = (dial - 1).rem_euclid(100);

                    if dial == 0 {
                        password += 1;
                    }
                }
            }
            'R' => {
                for _ in 0..step_amt {
                    dial = (dial + 1).rem_euclid(100);

                    if dial == 0 {
                        password += 1;
                    }
                }
            }
            _ => (),
        }
    }
    println!("The password with method 0x434C49434B is: {}", password);
}

pub fn old_method() {
    let data = fs::read_to_string("rotations.txt").unwrap();

    let instructions = data.lines();
    let mut dial: i32 = 50;
    let mut password = 0;

    for instruction in instructions {
        // For each instruction collect the direction L or R
        let direction = instruction.chars().next().unwrap();

        // For each instruction collect step amount
        let step_amt = instruction[1..].parse::<i32>().unwrap();

        // For each instruction if L subtract amt if R add amt to/from dial
        match direction {
            'L' => {
                dial = (dial - step_amt).rem_euclid(100);
            }
            'R' => {
                dial = (dial + step_amt).rem_euclid(100);
            }
            _ => (),
        }
        // Count how many 0's
        if dial == 0 {
            password += 1;
        }
    }
    println!("The old method password is: {}", password);
}
