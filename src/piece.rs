use std::cmp;
use std::fmt::Display;
#[derive(Debug, PartialEq, Eq)]
pub enum Player {
    White,
    Black,
}
pub const MAX_ROW: u8 = 8;
pub const MAX_COLUMN: u8 = MAX_ROW;
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Queen,
    King,
    Pawn,
    Bishop,
    Knight,
    Rook,
}

#[derive(Debug, PartialEq, Eq)]
pub enum KnightDirection {
    UpLeft,
    UpRight,
    RightUp,
    RightDown,
    DownLeft,
    DownRight,
    LeftUp,
    LeftDown,
}

#[derive(Debug)]
pub enum Direction {
    Up(u8),
    Left(u8),
    Down(u8),
    Right(u8),
    UpLeft(u8),
    UpRight(u8),
    DownLeft(u8),
    DownRight(u8),
    Knight(KnightDirection),
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
    pub fn get_direction(&self, other: &Self) -> Option<Direction> {
        let rmax = cmp::max(self.row, other.row);
        let rmin = cmp::min(self.row, other.row);
        let cmax = cmp::max(self.column, other.column);
        let cmin = cmp::min(self.column, other.column);
        let x = rmax - rmin;
        let y = cmax - cmin;
        if self.row == other.row {
            if self.column > other.column {
                return Some(Direction::Left(y));
            }
            return Some(Direction::Right(y));
        } else if self.column == other.column {
            if self.row > other.row {
                return Some(Direction::Down(x));
            }
            return Some(Direction::Up(x));
        } else if x == y {
            let direction = match (self.row > other.row, self.column > other.column) {
                (true, true) => Direction::DownLeft(x),
                (true, false) => Direction::DownRight(x),
                (false, true) => Direction::UpLeft(x),
                (false, false) => Direction::UpRight(x),
            };
            return Some(direction);
        } else if x == 2 && y == 1 {
            let direction = match (self.row > other.row, self.column > other.column) {
                (true, true) => KnightDirection::DownLeft,
                (true, false) => KnightDirection::DownRight,
                (false, true) => KnightDirection::UpLeft,
                (false, false) => KnightDirection::UpRight,
            };
            return Some(Direction::Knight(direction));
        } else if x == 1 && y == 2 {
            let direction = match (self.column > other.column, self.row > other.row) {
                (true, true) => KnightDirection::LeftDown,
                (true, false) => KnightDirection::LeftUp,
                (false, true) => KnightDirection::RightDown,
                (false, false) => KnightDirection::RightUp,
            };
            return Some(Direction::Knight(direction));
        }
        None
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

#[cfg(test)]
mod tests {
    use super::{Direction, KnightDirection, Position};

    #[test]
    fn test_direction_up() {
        let start = Position::new(3, 2);
        let end = Position::new(start.row + 1, start.column);
        let direction = start.get_direction(&end);
        if let Some(Direction::Up(_)) = direction {
            return;
        }
        panic!(
            "Going from {:?} to {:?} should be downward but got {:?}",
            start, end, direction
        );
    }

    #[test]
    fn test_direction_down() {
        let start = Position::new(3, 2);
        let end = Position::new(start.row - 1, start.column);
        let direction = start.get_direction(&end);
        if let Some(Direction::Down(_)) = direction {
            return;
        }
        panic!(
            "Going from {:?} to {:?} should be upward but got {:?}",
            start, end, direction
        );
    }

    #[test]
    fn test_direction_left() {
        let start = Position::new(3, 2);
        let end = Position::new(start.row, start.column - 1);
        let direction = start.get_direction(&end);
        if let Some(Direction::Left(_)) = direction {
            return;
        }
        panic!(
            "Going from {:?} to {:?} should be left sided but got {:?}",
            start, end, direction
        );
    }

    #[test]
    fn test_direction_right() {
        let start = Position::new(3, 2);
        let end = Position::new(start.row, start.column + 1);
        let direction = start.get_direction(&end);
        if let Some(Direction::Right(_)) = direction {
            return;
        }
        panic!(
            "Going from {:?} to {:?} should be right sided but got {:?}",
            start, end, direction
        );
    }

    #[test]
    fn test_direction_upleft() {
        let start = Position::new(3, 2);
        let end = Position::new(start.row + 1, start.column - 1);
        let direction = start.get_direction(&end);
        if let Some(Direction::UpLeft(_)) = direction {
            return;
        }
        panic!(
            "Going from {:?} to {:?} should be upleft but got {:?}",
            start, end, direction
        );
    }

    #[test]
    fn test_direction_upright() {
        let start = Position::new(3, 2);
        let end = Position::new(start.row + 1, start.column + 1);
        let direction = start.get_direction(&end);
        if let Some(Direction::UpRight(_)) = direction {
            return;
        }
        panic!(
            "Going from {:?} to {:?} should be upright but got {:?}",
            start, end, direction
        );
    }

    #[test]
    fn test_direction_downleft() {
        let start = Position::new(3, 2);
        let end = Position::new(start.row - 1, start.column - 1);
        let direction = start.get_direction(&end);
        if let Some(Direction::DownLeft(_)) = direction {
            return;
        }
        panic!(
            "Going from {:?} to {:?} should be downleft but got {:?}",
            start, end, direction
        );
    }

    #[test]
    fn test_direction_downright() {
        let start = Position::new(3, 2);
        let end = Position::new(start.row - 1, start.column + 1);
        let direction = start.get_direction(&end);
        if let Some(Direction::DownRight(_)) = direction {
            return;
        }
        panic!(
            "Going from {:?} to {:?} should be downleft but got {:?}",
            start, end, direction
        );
    }

    #[test]
    fn test_direction_knight() {
        use KnightDirection::*;
        let start = Position::new(4, 4);
        let modifiers: [(i8, i8, KnightDirection); 8] = [
            (2, 1, UpRight),
            (2, -1, UpLeft),
            (-2, 1, DownRight),
            (-2, -1, DownLeft),
            (1, 2, RightUp),
            (-1, 2, RightDown),
            (1, -2, LeftUp),
            (-1, -2, LeftDown),
        ];
        for (row, col, dir) in modifiers {
            let r: u8 = (i8::try_from(start.row).unwrap() + row).try_into().unwrap();
            let c: u8 = (i8::try_from(start.column).unwrap() + col)
                .try_into()
                .unwrap();
            let end = Position::new(r, c);
            let direction = start.get_direction(&end);
            let valid = match &direction {
                Some(Direction::Knight(kdir)) => kdir == &dir,
                _ => false,
            };
            if !valid {
                panic!(
                    "Going from {:?} to {:?} {:?} was expected but got {:?}",
                    start, end, dir, direction
                )
            }
        }
    }
}
