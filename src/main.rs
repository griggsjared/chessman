use chessman::Board;

fn main() {
    println!("Chessman!");
    let board = Board::startpos();
    println!("{}", board);
}
