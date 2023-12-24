fn gcd(a: u64, b: u64) -> u64 {
    if a == 0 {
        b
    } else if b == 0 {
        a
    } else if a > b {
        gcd(a % b, b)
    } else {
        gcd(a, b % a)
    }
}

pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
}
