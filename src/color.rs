#[allow(unused_imports)]
use std::str::FromStr;
use strum_macros::EnumString;

#[derive(Debug, PartialEq, EnumString, Clone, Copy)]
#[strum(ascii_case_insensitive)]
pub enum Color {
    Blue,
    Red,
    Cyan,
    Purple,
    Green,
    Orange,
    Pink,
    Grey,
    Lightblue,
    Brown,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enum_from_string() {
        assert_eq!(Color::from_str("red").unwrap(), Color::Red);
        assert_eq!(Color::from_str("Green").unwrap(), Color::Green);
        assert_eq!(Color::from_str("BLUE").unwrap(), Color::Blue);
    }
}
