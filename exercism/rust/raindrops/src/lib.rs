pub fn raindrops(n: u32) -> String {
    let mut s: String = String::new();
    let mut has_factor: bool = false;

    if n % 3 == 0 {
        s.push_str("Pling");
        has_factor = true;
    }

    if n % 5 == 0 {
        s.push_str("Plang");
        has_factor = true;
    }

    if n % 7 == 0 {
        s.push_str("Plong");
        has_factor = true;
    }

    if !has_factor {
        s = n.to_string()
    }
    s
}
