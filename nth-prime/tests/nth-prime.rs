use nth_prime as np;

#[test]
fn test_first_prime() {
    assert_eq!(np::nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(np::nth(1), 3);
}

#[test]
fn test_fourth_prime() {
    assert_eq!(np::nth(3), 7);
}

#[test]
fn test_prime_tester() {
    assert_eq!(np::is_prime(5), true);
    assert_eq!(np::is_prime(6), false);
    assert_eq!(np::is_prime(7), true);
    assert_eq!(np::is_prime(8), false);
    assert_eq!(np::is_prime(9), false);
    assert_eq!(np::is_prime(10), false);
    assert_eq!(np::is_prime(11), true);
    assert_eq!(np::is_prime(12), false);
    assert_eq!(np::is_prime(13), true);
}

#[test]
fn test_math() {
    let result: u32 = (5 as f64).sqrt().round() as u32;
    assert_eq!(result, 2);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(np::nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(np::nth(10_000), 104_743);
}
