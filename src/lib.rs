pub mod parser;

use parser::ParseSquareError;

#[derive(Debug, PartialEq)]
struct Square(char, u8);
struct Move;

#[derive(Debug)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}
#[derive(Debug)]
pub enum Player {
    White,
    Black,
}

pub struct Board {
    a: [Option<(Piece, Player)>; 8],
    b: [Option<(Piece, Player)>; 8],
    c: [Option<(Piece, Player)>; 8],
    d: [Option<(Piece, Player)>; 8],
    e: [Option<(Piece, Player)>; 8],
    f: [Option<(Piece, Player)>; 8],
    g: [Option<(Piece, Player)>; 8],
    h: [Option<(Piece, Player)>; 8],
}
impl Board {
    pub fn new() -> Board {
        Board {
            a: [
                Some((Piece::Rook, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Rook, Player::Black)),
            ],
            b: [
                Some((Piece::Knight, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Knight, Player::Black)),
            ],
            c: [
                Some((Piece::Bishop, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Bishop, Player::Black)),
            ],
            d: [
                Some((Piece::Queen, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Queen, Player::Black)),
            ],
            e: [
                Some((Piece::King, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::King, Player::Black)),
            ],
            f: [
                Some((Piece::Bishop, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Bishop, Player::Black)),
            ],
            g: [
                Some((Piece::Knight, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Knight, Player::Black)),
            ],
            h: [
                Some((Piece::Rook, Player::White)),
                Some((Piece::Pawn, Player::White)),
                None,
                None,
                None,
                None,
                Some((Piece::Pawn, Player::Black)),
                Some((Piece::Rook, Player::Black)),
            ],
        }
    }
    pub fn make_move(&mut self, m: &str) -> Result<&str, ()> {
        todo!()
    }
    pub fn check_square(&self, sq: &str) -> Result<Option<&(Piece, Player)>, ParseSquareError> {
        match sq.parse() {
            Ok(square) => match square {
                Square('a', i) => return Ok(self.a[(i - 1) as usize].as_ref()),
                Square('b', i) => return Ok(self.b[(i - 1) as usize].as_ref()),
                Square('c', i) => return Ok(self.c[(i - 1) as usize].as_ref()),
                Square('d', i) => return Ok(self.d[(i - 1) as usize].as_ref()),
                Square('e', i) => return Ok(self.e[(i - 1) as usize].as_ref()),
                Square('f', i) => return Ok(self.f[(i - 1) as usize].as_ref()),
                Square('g', i) => return Ok(self.g[(i - 1) as usize].as_ref()),
                Square('h', i) => return Ok(self.h[(i - 1) as usize].as_ref()),
                _ => unreachable!(),
            },
            Err(e) => return Err(e),
        }
    }
}
