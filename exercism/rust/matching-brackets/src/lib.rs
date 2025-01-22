pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '[' | '(' | '{' => stack.push(c),
            ']' => {
                if let Some(d) = stack.pop() {
                    if d != '[' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            ')' => {
                if let Some(d) = stack.pop() {
                    if d != '(' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            '}' => {
                if let Some(d) = stack.pop() {
                    if d != '{' {
                        return false;
                    }
                } else {
                    return false;
                }
            }
            _ => {}
        }
    }
    stack.is_empty()
}

/*
pub fn brackets_are_balanced(string: &str) -> bool {
    let mut brackets: Vec<char> = Vec::new();

    for c in string.chars() {
        match c {
            '(' | '{' | '[' => brackets.push(c),
            ')' => {
                if brackets.pop() != Some('(') {
                    return false;
                }
            }
            ']' => {
                if brackets.pop() != Some('[') {
                    return false;
                }
            }
            '}' => {
                if brackets.pop() != Some('{') {
                    return false;
                }
            }
            _ => (),
        }
    }
    brackets.is_empty()
}
*/
