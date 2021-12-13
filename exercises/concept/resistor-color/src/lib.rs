use enum_iterator::IntoEnumIterator;
use int_enum::IntEnum;

#[derive(Clone, Copy, Eq, Debug, IntoEnumIterator, PartialEq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}


#[repr(usize)]
#[derive(Clone, Copy, Eq, IntEnum, Debug, IntoEnumIterator, PartialEq)]
pub enum ResistorColorWithVal {
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
    match _color {
        ResistorColor::Black => 0,
        ResistorColor::Blue => 1,
        ResistorColor::Brown => 2,
        ResistorColor::Green => 3,
        ResistorColor::Grey => 4,
        ResistorColor::Orange => 5,
        ResistorColor::Red => 6,
        ResistorColor::Violet => 7,
        ResistorColor::White => 8,
        ResistorColor::Yellow => 9,
    }
}


pub fn value_to_color_string(value: usize) -> String {
    let color = ResistorColorWithVal::from_int(value).unwrap();
    // let color_str = match color {
    //     Ok(str) => str,
    //     Err(e) => e
    // };
    format!("{:?}", color)
}

pub fn colors() -> Vec<ResistorColor> {
    // unimplemented!("return a list of all the colors ordered by resistance")
    let mut vec = Vec::new();
    for color in ResistorColor::into_enum_iter() {
        vec.push(color);
    }
    vec
}
