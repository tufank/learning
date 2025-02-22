pub fn square(s: u32) -> u64 {
    if s < 1 || s > 64 {
        panic!("Square must be between 1 and 64")
    }
    (2u64).pow(s - 1)
}

pub fn total() -> u64 {
    // (1..=64).into_iter().fold(0, |acc, x| acc + square(x))
    ((2u128).pow(64) - 1) as u64
}
