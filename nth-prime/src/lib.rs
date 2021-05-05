use std::convert::TryFrom;

pub fn is_prime(n: u32) -> bool {
    match n {
        2 => true,
        3 => true,
        _ => {
            let max = u32::try_from((n as f64).sqrt().round() as u32).unwrap();
            let mut divisor = 2;

            while divisor <= max {
                if n % divisor == 0 {
                    return false;
                }
                divisor += 1;
            }
            true
        }
    }
}

pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        1 => 3,
        _ => {
            let mut result: u32 = 3;

            let mut current: u32 = 3;

            let mut i: u32 = 0;

            while i < n {
                if is_prime(current) {
                    result = current;
                    i += 1;
                }
                current += 2;
            }
            result
        }
    }
}
