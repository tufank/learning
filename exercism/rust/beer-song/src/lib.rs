pub fn verse(n: u32) -> String {
    match n {
        0 => {
            format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n", "No more", "s", "no more", "s")
        }
        _ => {
            let plural1 = if n == 1 { "" } else { "s" };
            let plural2 = if (n - 1) == 1 { "" } else { "s" };
            let one = if n == 1 { "it" } else { "one" };
            let left = if (n - 1) == 0 {
                String::from("no more")
            } else {
                (n - 1).to_string()
            };
            format!("{} bottle{} of beer on the wall, {} bottle{} of beer.\nTake {} down and pass it around, {} bottle{} of beer on the wall.\n", n, plural1, n, plural1, one, left, plural2)
        }
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let mut s = String::new();

    for n in (end..=start).rev() {
        let t = verse(n);
        s.push_str(t.as_str());

        if n != end {
            s.push_str("\n");
        }
    }
    s
}
