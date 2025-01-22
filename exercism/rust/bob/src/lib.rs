use self::BobResponse::*;

pub enum BobResponse {
    Sure,
    ChillOut,
    CalmDown,
    Fine,
    Whatever,
}

pub fn reply(message: &str) -> &str {
    match get_response(message) {
        Sure => "Sure.",
        ChillOut => "Whoa, chill out!",
        CalmDown => "Calm down, I know what I'm doing!",
        Fine => "Fine. Be that way!",
        Whatever => "Whatever.",
    }
}

pub fn get_response(message: &str) -> BobResponse {
    let m = message.trim();
    let mut all_caps = true;
    let mut no_letters = true;

    for i in m.chars() {
        if i.is_alphabetic() {
            no_letters = false;
        }
        if i.is_lowercase() {
            all_caps = false;
        }
    }

    if m.is_empty() {
        Fine
    } else {
        if all_caps && !no_letters {
            if m.ends_with("?") {
                CalmDown
            } else {
                ChillOut
            }
        } else {
            if m.ends_with("?") {
                Sure
            } else {
                Whatever
            }
        }
    }
}

/*
fn is_yelling(message: &str) -> bool {
    let have_letters: bool = message.chars().filter(|x| x.is_alphabetic()).count() > 0;

    message.to_uppercase() == message && have_letters
}

pub fn reply(message: &str) -> &str {
    match message.trim() {
        m if m.trim().len() == 0 => "Fine. Be that way!",

        m if m.ends_with("?") && is_yelling(m) => "Calm down, I know what I'm doing!",

        m if m.ends_with("?") => "Sure.",

        m if is_yelling(m) => "Whoa, chill out!",

        _ => "Whatever.",
    }
}
*/
