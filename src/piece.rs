use std::fmt::Display;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Queen,
    King,
    Pawn,
    Bishop,
    Knight,
    Rook,
}

pub enum MoveType {
    Straight,
    Diagonal,
    StraightAndDiagonal,
    Knight,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Position {
    pub row: u8,
    pub column: u8,
}

impl Position {
    pub fn from_piece(piece: &Piece) -> Self {
        Self {
            row: piece.row,
            column: piece.column,
        }
    }
    pub fn new(row: u8, column: u8) -> Self {
        Self { row, column }
    }
}

impl From<Kind> for MoveType {
    fn from(value: Kind) -> Self {
        match value {
            Kind::Rook => MoveType::Straight,
            Kind::Pawn | Kind::Queen | Kind::King => MoveType::StraightAndDiagonal,
            Kind::Bishop => MoveType::Diagonal,
            Kind::Knight => MoveType::Knight,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Piece {
    pub kind: Kind,
    pub row: u8,
    pub column: u8,
}
impl Piece {
    pub fn r#move(&mut self, row: u8, column: u8) {
        self.column = column;
        self.row = row;
    }
    pub fn new(kind: Kind, row: u8, column: u8) -> Self {
        Self { kind, row, column }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val: &str = match self.kind {
            Kind::King => "♔",
            Kind::Queen => "♕",
            Kind::Bishop => "♗",
            Kind::Knight => "♘",
            Kind::Rook => "♖",
            Kind::Pawn => "♙",
        };
        write!(f, "{}", val)
    }
}
