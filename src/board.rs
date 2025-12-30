use std::fmt::{Display, Error, Formatter};
use crate::types::{Piece, PieceColor, PieceKind, Square};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    squares: [Option<Piece>; 64],
    side_to_move: PieceColor,
}

impl Board {
    pub fn new() -> Board {
        Board {
            squares: [None; 64],
            side_to_move: PieceColor::White,
        }
    }

    pub fn piece_at(&self, square: Square) -> Option<Piece> {
        self.squares[square.index()]
    }

    pub fn side_to_move(&self) -> PieceColor {
        self.side_to_move
    }

    pub fn set_piece_at(&mut self, square: Square, piece: Option<Piece>) {
        self.squares[square.index()] = piece;
    }

    pub fn startpos() -> Self {
        let mut board = Self::new();
        let back_rank = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ];

        for (file, kind) in back_rank.into_iter().enumerate() {
            let file = file as u8;
            // White pieces (bottom side)
            board.set_piece_at(
                Square::from_rank_and_file(0, file).unwrap(),
                Some(Piece::new(PieceColor::White, kind)),
            );
            board.set_piece_at(
                Square::from_rank_and_file(1, file).unwrap(),
                Some(Piece::new(PieceColor::White, PieceKind::Pawn)),
            );

            // Black pieces (Top side)
            board.set_piece_at(
                Square::from_rank_and_file(7, file).unwrap(),
                Some(Piece::new(PieceColor::Black, kind)),
            );
            board.set_piece_at(
                Square::from_rank_and_file(6, file).unwrap(),
                Some(Piece::new(PieceColor::Black, PieceKind::Pawn)),
            );
        }

        board
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        for rank in (0..8).rev() {
            write!(f, "{} ", rank + 1)?;
            for file in 0..8 {
                let square = Square::from_rank_and_file(rank, file).unwrap();
                let char = self.piece_at(square).map_or('.', |p| p.to_fen());
                write!(f, "{} ", char)?;
            }
            writeln!(f)?;
        }
        writeln!(f, "  a b c d e f g h")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_new() {
        let board = Board::new();
        for index in 0..64 {
            let square = Square::from_index(index).unwrap();
            assert_eq!(board.piece_at(square), None);
        }
        assert_eq!(board.side_to_move(), PieceColor::White);
    }

    #[test]
    fn test_board_set_and_get_piece() {
        let mut board = Board::new();
        let square = Square::from_rank_and_file(0, 0).unwrap();
        let piece = Piece::new(PieceColor::White, PieceKind::Rook);
        board.set_piece_at(square, Some(piece));
        assert_eq!(board.piece_at(square), Some(piece));
    }

    #[test]
    fn test_board_startpos() {
        let board = Board::startpos();
        assert_eq!(
            board.piece_at(Square::from_rank_and_file(0, 0).unwrap()),
            Some(Piece::new(PieceColor::White, PieceKind::Rook))
        );
        assert_eq!(
            board.piece_at(Square::from_rank_and_file(7, 4).unwrap()),
            Some(Piece::new(PieceColor::Black, PieceKind::King))
        );
        assert_eq!(
            board.piece_at(Square::from_rank_and_file(1, 3).unwrap()),
            Some(Piece::new(PieceColor::White, PieceKind::Pawn))
        );
        assert_eq!(
            board.piece_at(Square::from_rank_and_file(6, 5).unwrap()),
            Some(Piece::new(PieceColor::Black, PieceKind::Pawn))
        );
        assert_eq!(board.piece_at(Square::from_rank_and_file(4, 4).unwrap()), None);
    }

    #[test]
    fn test_board_display() {
        let board = Board::startpos();
        let board_str = format!("{}", board);
        let expected_str = concat!(
            "8 r n b q k b n r \n",
            "7 p p p p p p p p \n",
            "6 . . . . . . . . \n",
            "5 . . . . . . . . \n",
            "4 . . . . . . . . \n",
            "3 . . . . . . . . \n",
            "2 P P P P P P P P \n",
            "1 R N B Q K B N R \n",
            "  a b c d e f g h\n",
        );
        assert_eq!(board_str, expected_str);
    }
}
