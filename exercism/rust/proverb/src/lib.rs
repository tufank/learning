pub fn build_proverb(list: &[&str]) -> String {
    let mut s = String::new();

    if list.len() > 0 {
        for i in list.windows(2) {
            s.push_str(format!("For want of a {} the {} was lost.\n", i[0], i[1]).as_str());
        }
        s.push_str(format!("And all for the want of a {}.", list.iter().next().unwrap()).as_str());
    }
    s
}
