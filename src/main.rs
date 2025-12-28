use chess::{Piece, PieceColor, PieceKind};

fn main() {
    println!("Chess!");
    println!(
        "White King in FEN: {}",
        Piece::new(PieceColor::White, PieceKind::King).to_fen()
    );

    println!(
        "Black Queen from FEN 'q': {:?}",
        Piece::from_fen('q')
    );

    println!(
        "Opposite of White is {:?}",
        PieceColor::White.opposite()
    );
}
