use std::collections::HashMap;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut numbers = HashMap::new();

    for i in factors {
        if *i == 0 {
            continue;
        }

        for j in (1..limit).into_iter().filter(|x| x % i == 0) {
            numbers.entry(j).or_insert(1);
        }
    }

    numbers.keys().sum()
}
