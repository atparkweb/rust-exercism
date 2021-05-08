// use std::convert::TryFrom;

pub fn is_prime(n: u32) -> bool {
    match n {
        2 => true,
        3 => true,
        _ => {
            let end: u32 = ((n as f32).sqrt()) as u32;
            for i in 2..(end + 1) {
                if n % i == 0 {
                    return false;
                }
            }
            true
        }
    }
}

pub fn nth(n: u32) -> u32 {
    let mut current = 2;

    let mut result = current;

    let mut i = 0;

    while i < n {
        current += 1;

        if is_prime(current) {
            result = current;
            i += 1;
        }
    }
    result
}
