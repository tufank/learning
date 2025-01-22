use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut maga_hash_map: HashMap<String, u32> = HashMap::new();
    let mut note_hash_map: HashMap<String, u32> = HashMap::new();

    for i in note {
        let count = note_hash_map.entry(i.to_string()).or_insert(0);
        *count += 1;
    }

    for i in magazine {
        let count = maga_hash_map.entry(i.to_string()).or_insert(0);
        *count += 1;
    }

    for (key, value) in note_hash_map {
        if !maga_hash_map.contains_key(&key) {
            return false;
        }

        if value > maga_hash_map[&key] {
            return false;
        }
    }

    return true;
}
