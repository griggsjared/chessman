use crate::types::{Piece, PieceColor, PieceKind, Square};
use std::{
    fmt::{Display, Error, Formatter},
    str::FromStr,
};

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

    pub fn from_fen(fen: &str) -> Result<Self, ParseFenError> {
        Self::from_str(fen)
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

#[derive(Debug, PartialEq, Eq)]
pub struct ParseFenError(String);

impl FromStr for Board {
    type Err = ParseFenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut board = Board::new();

        let parts: Vec<&str> = s.split_whitespace().collect();

        if parts.len() < 2 {
            return Err(ParseFenError("Invalid FEN: Not enough parts".to_string()));
        }

        let ranks: Vec<&str> = parts[0].split('/').collect();

        if ranks.len() != 8 {
            return Err(ParseFenError(
                "Invalid FEN: Incorrect number of ranks".to_string(),
            ));
        }

        // Each rank must expand to exactly 8 files.
        for (rank_index, rank) in ranks.iter().enumerate() {
            let mut file_count: usize = 0;

            for c in rank.chars() {
                if matches!(c, '1'..='8') {
                    file_count += c.to_digit(10).unwrap() as usize;
                } else {
                    file_count += 1;
                }
            }

            if file_count != 8 {
                return Err(ParseFenError(format!(
                    "Invalid FEN: Rank {} does not expand to 8 squares",
                    8 - rank_index
                )));
            }
        }

        for (rank_index, rank_str) in ranks.iter().enumerate() {
            // file index 0 to 7
            let mut file_index = 0;
            for c in rank_str.chars() {
                // If adding the next piece/squares would go past the last file,
                // bail out with an explicit error.
                if file_index > 7 {
                    return Err(ParseFenError(format!(
                        "Invalid FEN: Too many squares in rank {}",
                        8 - rank_index
                    )));
                }

                if matches!(c, '1'..='8') {
                    let empty_squares = c.to_digit(10).unwrap() as usize;
                    file_index += empty_squares;
                } else {
                    let piece = Piece::from_fen(c).ok_or(ParseFenError(format!(
                        "Invalid FEN: Unknown piece character '{}'",
                        c
                    )))?;
                    let square = Square::from_rank_and_file(7 - rank_index as u8, file_index as u8)
                        .ok_or(ParseFenError(
                            "Invalid FEN: Square out of bounds".to_string(),
                        ))?;
                    board.set_piece_at(square, Some(piece));
                    file_index += 1;
                }
            }

            if file_index != 8 {
                return Err(ParseFenError(format!(
                    "Invalid FEN: Rank {} does not end on file 8",
                    8 - rank_index
                )));
            }
        }

        board.side_to_move = match parts[1] {
            "w" => PieceColor::White,
            "b" => PieceColor::Black,
            _ => {
                return Err(ParseFenError(
                    "Invalid FEN: Invalid side to move".to_string(),
                ));
            }
        };

        Ok(board)
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
        assert_eq!(
            board.piece_at(Square::from_rank_and_file(4, 4).unwrap()),
            None
        );
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

    #[test]
    fn test_board_from_starting_position_fen() {
        // starting position FEN
        let fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w";
        let board = Board::from_str(fen).unwrap();
        assert_eq!(
            board.piece_at(Square::from_rank_and_file(0, 0).unwrap()),
            Some(Piece::new(PieceColor::White, PieceKind::Rook))
        );
        assert_eq!(
            board.piece_at(Square::from_rank_and_file(7, 4).unwrap()),
            Some(Piece::new(PieceColor::Black, PieceKind::King))
        );
        assert_eq!(board.side_to_move(), PieceColor::White);
    }

    #[test]
    fn test_board_from_custom_fen() {
        // black king on d5, white king on e2, black to move
        let fen = "8/8/8/3k4/8/8/4K3/8 b";
        let board = Board::from_str(fen).unwrap();
        assert_eq!(
            board.piece_at(Square::from_rank_and_file(4, 3).unwrap()),
            Some(Piece::new(PieceColor::Black, PieceKind::King))
        );
        assert_eq!(
            board.piece_at(Square::from_rank_and_file(1, 4).unwrap()),
            Some(Piece::new(PieceColor::White, PieceKind::King))
        );
        assert_eq!(board.side_to_move(), PieceColor::Black);
    }

    #[test]
    fn test_board_from_fen_invalid() {
        let invalid_fen = "invalid_fen_string";
        let result = Board::from_str(invalid_fen);
        assert!(result.is_err());
    }

    #[test]
    fn test_board_from_fen_incorrect_ranks() {
        let invalid_fen = "8/8/8/8/8/8/8 w"; // only 7 ranks
        let result = Board::from_str(invalid_fen);
        assert!(result.is_err());
    }

    #[test]
    fn test_board_from_fen_invalid_side_to_move() {
        let invalid_fen = "8/8/8/8/8/8/8/8 x"; // invalid side to move
        let result = Board::from_str(invalid_fen);
        assert!(result.is_err());
    }

    #[test]
    fn test_board_from_fen_invalid_rank_width() {
        let invalid_fen = "9/8/8/8/8/8/8/8 w"; // rank with 9 squares
        let result = Board::from_str(invalid_fen);
        assert!(result.is_err());
    }

    #[test]
    fn test_board_from_fen_unknown_piece() {
        let invalid_fen = "8/8/8/8/8/8/8/7X w"; // 'X' is not a valid piece
        let result = Board::from_str(invalid_fen);
        assert!(result.is_err());
    }
}
