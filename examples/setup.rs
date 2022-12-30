use chess_rs::Board;

fn main() {
    let mut board = Board::new();
    dbg!(board.check_square("g1".to_string()));
    dbg!(board.check_square("f3".to_string()));
    board.r#move("Nf3".to_string());
    dbg!(board.check_square("g1".to_string()));
    dbg!(board.check_square("f3".to_string()));
}
