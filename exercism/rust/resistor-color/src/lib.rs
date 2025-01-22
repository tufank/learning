use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;
use std::fmt;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, IntEnum, IntoEnumIterator)]
pub enum ResistorColor {
    Black = 0,
    Blue = 6,
    Brown = 1,
    Green = 5,
    Grey = 8,
    Orange = 3,
    Red = 2,
    Violet = 7,
    White = 9,
    Yellow = 4,
}

impl fmt::Display for ResistorColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
        // or, alternatively:
        // fmt::Debug::fmt(self, f)
    }
}

pub fn color_to_value(color: ResistorColor) -> usize {
    color.int_value() as usize
}

pub fn value_to_color_string(value: usize) -> String {
    match ResistorColor::into_enum_iter().find(|&x| value == color_to_value(x)) {
        Some(x) => x.to_string(),
        None => String::from("value out of range"),
    }
}

pub fn colors() -> Vec<ResistorColor> {
    (0..=9)
        .map(|x| ResistorColor::from_int(x).unwrap())
        .collect::<Vec<ResistorColor>>()
}
