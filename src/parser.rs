use std::str::FromStr;

use crate::Move;
use crate::Square;

#[derive(Debug, PartialEq)]
pub struct ParseSquareError;

impl FromStr for Square {
    type Err = ParseSquareError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return Err(ParseSquareError);
        }

        let file = match chars[0] {
            'a' => 'a',
            'b' => 'b',
            'c' => 'c',
            'd' => 'd',
            'e' => 'e',
            'f' => 'f',
            'g' => 'g',
            'h' => 'h',
            _ => return Err(ParseSquareError),
        };
        let rank = match chars[1] {
            '1' => 1,
            '2' => 2,
            '3' => 3,
            '4' => 4,
            '5' => 5,
            '6' => 6,
            '7' => 7,
            '8' => 8,
            _ => return Err(ParseSquareError),
        };

        Ok(Self(file, rank))
    }
}

impl FromStr for Move {
    type Err = ParseSquareError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use crate::{parser::ParseSquareError, Square};

    #[test]
    fn parsing_a1_works() {
        assert_eq!("a1".parse::<Square>().unwrap(), Square('a', 1));
    }
    #[test]
    fn parsing_d4_works() {
        assert_eq!("d4".parse::<Square>().unwrap(), Square('d', 4));
    }
    #[test]
    fn parsing_h8_works() {
        assert_eq!("h8".parse::<Square>().unwrap(), Square('h', 8));
    }
    #[test]
    fn parsing_i8_fails() {
        assert_eq!("i8".parse::<Square>().unwrap_err(), ParseSquareError);
    }
    #[test]
    fn parsing_h9_fails() {
        assert_eq!("i8".parse::<Square>().unwrap_err(), ParseSquareError);
    }
}
