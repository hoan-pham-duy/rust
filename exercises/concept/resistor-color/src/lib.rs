use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[repr(u8)]
#[derive(Clone, Copy, Eq, IntEnum, Debug, IntoEnumIterator, PartialEq)]
pub enum ResistorColor {
    Black = 0,
    Blue = 1,
    Brown = 2,
    Green = 3,
    Grey = 4,
    Orange = 5,
    Red = 6,
    Violet = 7,
    White = 8,
    Yellow = 9,
}

pub fn color_to_value(_color: ResistorColor) -> usize {
    _color.int_value() as usize
}

pub fn value_to_color_string(value: usize) -> String {
    // unimplemented!(
    //     "convert the value {} into a string representation of color",
    //     value
    // )
    format!("{:#?}", ResistorColor::from_int(value.try_into().unwrap()))
}

pub fn colors() -> Vec<ResistorColor> {
    // unimplemented!("return a list of all the colors ordered by resistance")
    let mut vec = Vec::new();
    for color in ResistorColor::into_enum_iter() {
        vec.push(color);
    }
    vec
}
