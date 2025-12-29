use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

impl PieceColor {
    pub fn opposite(self) -> Self {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PieceKind {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Piece {
    color: PieceColor,
    kind: PieceKind,
}

impl Piece {
    /// Creates a new Piece with the given color and kind
    pub fn new(color: PieceColor, kind: PieceKind) -> Piece {
        Piece { color, kind }
    }

    /// Creates a `Piece` from a FEN character
    /// FEN notation uses uppercase letters for White pieces and lowercase for Black pieces
    /// and the letters are as follows:
    /// - `K`/`k`: King
    /// - `Q`/`q`: Queen
    /// - `R`/`r`: Rook
    /// - `B`/`b`: Bishop
    /// - `N`/`n`: Knight
    /// - `P`/`p`: Pawn
    /// Returns Option<Piece> which is Some(Piece) if the character is valid, or None if invalid
    pub fn from_fen(fen: char) -> Option<Piece> {
        match fen {
            'K' => Some(Piece::new(PieceColor::White, PieceKind::King)),
            'k' => Some(Piece::new(PieceColor::Black, PieceKind::King)),
            'Q' => Some(Piece::new(PieceColor::White, PieceKind::Queen)),
            'q' => Some(Piece::new(PieceColor::Black, PieceKind::Queen)),
            'R' => Some(Piece::new(PieceColor::White, PieceKind::Rook)),
            'r' => Some(Piece::new(PieceColor::Black, PieceKind::Rook)),
            'B' => Some(Piece::new(PieceColor::White, PieceKind::Bishop)),
            'b' => Some(Piece::new(PieceColor::Black, PieceKind::Bishop)),
            'N' => Some(Piece::new(PieceColor::White, PieceKind::Knight)),
            'n' => Some(Piece::new(PieceColor::Black, PieceKind::Knight)),
            'P' => Some(Piece::new(PieceColor::White, PieceKind::Pawn)),
            'p' => Some(Piece::new(PieceColor::Black, PieceKind::Pawn)),
            _ => None,
        }
    }

    /// Converts the Piece to its FEN character representation
    pub fn to_fen(&self) -> char {
        match (self.color, &self.kind) {
            (PieceColor::White, PieceKind::King) => 'K',
            (PieceColor::Black, PieceKind::King) => 'k',
            (PieceColor::White, PieceKind::Queen) => 'Q',
            (PieceColor::Black, PieceKind::Queen) => 'q',
            (PieceColor::White, PieceKind::Rook) => 'R',
            (PieceColor::Black, PieceKind::Rook) => 'r',
            (PieceColor::White, PieceKind::Bishop) => 'B',
            (PieceColor::Black, PieceKind::Bishop) => 'b',
            (PieceColor::White, PieceKind::Knight) => 'N',
            (PieceColor::Black, PieceKind::Knight) => 'n',
            (PieceColor::White, PieceKind::Pawn) => 'P',
            (PieceColor::Black, PieceKind::Pawn) => 'p',
        }
    }

    pub fn color(self) -> PieceColor {
        self.color
    }

    pub fn kind(self) -> PieceKind {
        self.kind
    }
}

/// Struct representing a square and its position on the chessboard
/// Uses 0-63 to represent squares from a1 (0) to h8 (63)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Square(u8);

impl Square {
    /// Returns a new Square given rank and file (0-indexed)
    pub fn new(rank: u8, file: u8) -> Option<Square> {
        if rank < 8 && file < 8 {
            Some(Square(rank * 8 + file))
        } else {
            None
        }
    }

    /// Returns the file (0-7) of the square
    pub fn file(self) -> u8 {
        self.0 % 8
    }

    /// Returns the rank (0-7) of the square
    pub fn rank(self) -> u8 {
        self.0 / 8
    }

    /// Returns the index (0-63) of the square
    pub fn index(self) -> usize {
        self.0 as usize
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ParseSquareError;

impl FromStr for Square {
    type Err = ParseSquareError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // TODO: maybe use as_bytes for performance?
        if s.len() != 2 {
            return Err(ParseSquareError);
        }

        let file_char = s.chars().nth(0).unwrap();
        let rank_char = s.chars().nth(1).unwrap();

        let file = match file_char {
            'a'..='h' => (file_char as u8) - ('a' as u8),
            _ => return Err(ParseSquareError),
        };

        let rank = match rank_char {
            '1'..='8' => (rank_char as u8) - ('1' as u8),
            _ => return Err(ParseSquareError),
        };

        let square = Square::new(rank, file).ok_or(ParseSquareError)?;

        Ok(square)
    }
}

impl Display for Square {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(
            f,
            "{}{}",
            (self.file() + b'a') as char,
            (self.rank() + b'1') as char
        )
    }
}

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
                Square::new(0, file).unwrap(),
                Some(Piece::new(PieceColor::White, kind)),
            );
            board.set_piece_at(
                Square::new(1, file).unwrap(),
                Some(Piece::new(PieceColor::White, PieceKind::Pawn)),
            );

            // Black pieces (Top side)
            board.set_piece_at(
                Square::new(7, file).unwrap(),
                Some(Piece::new(PieceColor::Black, kind)),
            );
            board.set_piece_at(
                Square::new(6, file).unwrap(),
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
                let square = Square::new(rank, file).unwrap();
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
    fn test_color_opposite() {
        assert_eq!(PieceColor::White.opposite(), PieceColor::Black);
        assert_eq!(PieceColor::Black.opposite(), PieceColor::White);
    }

    #[test]
    fn test_piece_from_fen() {
        for ch in ['K', 'k', 'Q', 'q', 'R', 'r', 'B', 'b', 'N', 'n', 'P', 'p'] {
            let p = Piece::from_fen(ch).unwrap();
            assert_eq!(p.to_fen(), ch);
        }
        let invalid_piece = Piece::from_fen('Z');
        assert!(invalid_piece.is_none());
    }

    #[test]
    fn test_piece_to_fen() {
        for p in [
            Piece::new(PieceColor::White, PieceKind::King),
            Piece::new(PieceColor::Black, PieceKind::King),
            Piece::new(PieceColor::White, PieceKind::Queen),
            Piece::new(PieceColor::Black, PieceKind::Queen),
            Piece::new(PieceColor::White, PieceKind::Rook),
            Piece::new(PieceColor::Black, PieceKind::Rook),
            Piece::new(PieceColor::White, PieceKind::Bishop),
            Piece::new(PieceColor::Black, PieceKind::Bishop),
            Piece::new(PieceColor::White, PieceKind::Knight),
            Piece::new(PieceColor::Black, PieceKind::Knight),
            Piece::new(PieceColor::White, PieceKind::Pawn),
            Piece::new(PieceColor::Black, PieceKind::Pawn),
        ] {
            let fen_char = p.to_fen();
            let reconstructed_piece = Piece::from_fen(fen_char).unwrap();
            assert_eq!(p, reconstructed_piece);
        }
    }

    #[test]
    fn test_piece_color_and_kind() {
        let piece = Piece::new(PieceColor::White, PieceKind::Queen);
        assert_eq!(piece.color(), PieceColor::White);
        assert_eq!(piece.kind(), PieceKind::Queen);
    }

    #[test]
    fn test_square_from_rank_and_file() {
        let square = Square::new(0, 0).unwrap();
        assert_eq!(square.rank(), 0);
        assert_eq!(square.file(), 0);

        let square = Square::new(7, 7).unwrap();
        assert_eq!(square.rank(), 7);
        assert_eq!(square.file(), 7);

        let invalid_square = Square::new(8, 0);
        assert!(invalid_square.is_none());
    }

    #[test]
    fn test_square_rank_file() {
        let square = Square(27);
        assert_eq!(square.rank(), 3);
        assert_eq!(square.file(), 3);
    }

    #[test]
    fn test_square_index() {
        let square = Square::new(4, 5).unwrap();
        assert_eq!(square.index(), 37);
    }

    #[test]
    fn test_square_from_str() {
        let square: Square = "e4".parse().unwrap();
        assert_eq!(square.rank(), 3);
        assert_eq!(square.file(), 4);

        let invalid_square: Result<Square, _> = "z9".parse();
        assert!(invalid_square.is_err());
    }

    #[test]
    fn test_square_display() {
        let square = Square::new(0, 0).unwrap();
        assert_eq!(square.to_string(), "a1");

        let square = Square::new(7, 7).unwrap();
        assert_eq!(square.to_string(), "h8");
    }

    #[test]
    fn test_board_new() {
        let board = Board::new();
        for index in 0..64 {
            let square = Square(index as u8);
            assert_eq!(board.piece_at(square), None);
        }
        assert_eq!(board.side_to_move(), PieceColor::White);
    }

    #[test]
    fn test_board_set_and_get_piece() {
        let mut board = Board::new();
        let square = Square::new(0, 0).unwrap();
        let piece = Piece::new(PieceColor::White, PieceKind::Rook);
        board.set_piece_at(square, Some(piece));
        assert_eq!(board.piece_at(square), Some(piece));
    }

    #[test]
    fn test_board_startpos() {
        let board = Board::startpos();
        assert_eq!(
            board.piece_at(Square::new(0, 0).unwrap()),
            Some(Piece::new(PieceColor::White, PieceKind::Rook))
        );
        assert_eq!(
            board.piece_at(Square::new(7, 4).unwrap()),
            Some(Piece::new(PieceColor::Black, PieceKind::King))
        );
        assert_eq!(
            board.piece_at(Square::new(1, 3).unwrap()),
            Some(Piece::new(PieceColor::White, PieceKind::Pawn))
        );
        assert_eq!(
            board.piece_at(Square::new(6, 5).unwrap()),
            Some(Piece::new(PieceColor::Black, PieceKind::Pawn))
        );
        assert_eq!(board.piece_at(Square::new(4, 4).unwrap()), None);
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
