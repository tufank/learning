pub fn is_prime(n: u32) -> bool {
    let mut v = Vec::new();
    let mut n = n;
    let mut d: u32 = 3;

    while n % 2 == 0 {
        n = n / 2;
        v.push(2);
    }

    while d <= (n as f32).sqrt() as u32 {
        while n % d == 0 {
            v.push(d);
            n = n / d;
        }
        d += 2;
    }

    if n > 2 {
        v.push(n);
    }

    v.len() == 1
}

pub fn nth(n: u32) -> u32 {
    let mut count = 1;
    let mut num = 1;

    if n == 0 {
        return 2;
    }

    while count <= n {
        num += 2;
        if is_prime(num) {
            count += 1;
        }
    }
    num
}
