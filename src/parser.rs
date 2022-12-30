use std::str::FromStr;

use crate::Square;

#[derive(Debug)]
pub enum ParseError {
    Invalid,
    NotFound,
}
impl FromStr for Square {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        if chars.len() != 2 {
            return Err(ParseError::Invalid);
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
            _ => return Err(ParseError::NotFound),
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
            _ => return Err(ParseError::NotFound),
        };

        Ok(Square(file, rank))
    }
}
