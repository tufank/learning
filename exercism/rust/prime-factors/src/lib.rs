pub fn factors(n: u64) -> Vec<u64> {
    let mut v = Vec::new();
    let mut n = n;
    let mut d: u64 = 3;

    while n % 2 == 0 {
        n = n / 2;
        v.push(2);
    }

    while d <= (n as f64).sqrt() as u64 {
        while n % d == 0 {
            v.push(d);
            n = n / d;
        }
        d += 2;
    }

    if n > 2 {
        v.push(n);
    }
    v
}
