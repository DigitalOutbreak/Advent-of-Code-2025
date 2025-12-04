pub fn is_repeated_pattern(num: u64) -> bool {
    let num = num.to_string();
    let (split1, split2) = num.split_at(num.len() / 2);
    if split1 == split2 { true } else { false }
}

pub fn is_new_repeated_pattern(num: u64) -> bool {
    let num = num.to_string();
    let len = num.len();

    if len < 2 {
        return false;
    }

    for slice_size in 1..=len / 2 {
        if len % slice_size != 0 {
            continue;
        }

        let pattern = &num[0..slice_size];

        let mut is_valid = true;
        let mut start = slice_size;

        while start < len {
            let end = start + slice_size;
            let current = &num[start..end];

            if current != pattern {
                is_valid = false;
                break;
            }

            start += slice_size;
        }

        if is_valid {
            return true;
        }
    }

    false
}
