#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn vec_compare<T: PartialEq>(first: &[T], second: &[T]) -> bool {
    (first.len() == second.len()) && first.iter().zip(second).all(|(x, y)| x == y)
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    let flen = _first_list.len();
    let slen = _second_list.len();
    let mut result = 3;

    if flen == slen {
        if vec_compare(_first_list, _second_list) {
            result = 0;
        }
    } else if flen < slen {
        let diff = slen - flen;
        for i in 0..=diff {
            if vec_compare(&_second_list[i..i + flen], _first_list) {
                result = 1;
                break;
            }
        }
    } else {
        let diff = flen - slen;
        for i in 0..=diff {
            if vec_compare(&_first_list[i..i + slen], _second_list) {
                result = 2;
                break;
            }
        }
    }

    match result {
        0 => Comparison::Equal,
        1 => Comparison::Sublist,
        2 => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
