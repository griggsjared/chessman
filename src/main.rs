use chessman::Board;

fn main() {
    println!("Chessman!");

    let board = Board::from_fen("8/8/8/2Qk4/8/8/4Pq2/8 b")
        .unwrap_or_else(|e| panic!("Failed to create board from FEN: {:?}", e));

    println!("{}", board);
}
