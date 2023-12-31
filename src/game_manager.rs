use crate::{
    move_validators::is_valid_move,
    piece::{Kind, Piece, Player, Position, MAX_COLUMN, MAX_ROW},
};
use std::cmp;

#[derive(Debug)]
enum MoveErr {
    SamePosition,
    FriendlyFire,
    InvalidMove,
    PieceBlocking,
}

#[derive(Debug)]
pub struct GameManager {
    pub whites: Vec<Piece>,
    pub blacks: Vec<Piece>,
    pub turn: Player,
}

impl GameManager {
    pub fn new() -> Self {
        let whites = create_whites();
        let blacks = create_blacks_from_whites(&whites);
        Self {
            whites,
            blacks,
            turn: Player::White,
        }
    }

    pub fn swap_turn(&mut self) {
        self.turn = match self.turn {
            Player::Black => Player::White,
            Player::White => Player::Black,
        };
    }

    fn is_valid_move(&self, piece: &Piece, end: &Position) -> Option<MoveErr> {
        let pieces = self.blacks.iter().chain(self.whites.iter());
        let positions: Vec<Position> = pieces
            .filter(|p| !(p.row == piece.row && p.column == piece.column))
            .map(Position::from_piece)
            .collect();

        let start = &Position::from_piece(piece);

        if start == end {
            return Some(MoveErr::SamePosition);
        } else if is_friendly_fire(self, end) {
            return Some(MoveErr::FriendlyFire);
        }
        if !is_valid_move(piece, end, &self.turn) {
            return Some(MoveErr::InvalidMove);
        } else if self.is_piece_blocking(piece, end) {
            return Some(MoveErr::PieceBlocking);
        }

        // piece blocking
        None
    }
    pub fn move_piece(&mut self, piece: &Piece, pos: Position) -> Result<(), MoveErr> {
        if let Some(err) = self.is_valid_move(piece, &pos) {
            return Err(err);
        }
        Ok(())
    }

    pub fn move_suggestion(&self, piece: &Piece) -> Vec<Position> {
        let mut positions = vec![];
        for ci in 0..MAX_COLUMN {
            for ri in 0..MAX_ROW {
                let pos = Position::new(ri, ci);
                if let Some(err) = self.is_valid_move(piece, &pos) {
                    eprintln!("{:?}", err);
                } else {
                    positions.push(pos);
                }
            }
        }
        positions
    }
    fn is_piece_blocking(&self, piece: &Piece, end: &Position) -> bool {
        use crate::piece::Direction::*;

        let pieces = self.whites.iter().chain(self.blacks.iter());
        let positions: Vec<Position> = pieces.map(Position::from_piece).collect();
        let start = &Position::from_piece(piece);
        let direction = start.get_direction(end).unwrap();
        for pos in positions {
            if pos.row == piece.row && pos.column == piece.column {
                continue;
            } else if !is_valid_move(piece, &pos, &self.turn) {
                continue;
            };
            let dir = start.get_direction(&pos).unwrap();
            let is_blocking = match (&dir, &direction) {
                (Up(a), Up(b)) => a < b,
                (Left(a), Left(b)) => a < b,
                (Down(a), Down(b)) => a < b,
                (Right(a), Right(b)) => a < b,
                (UpLeft(a), UpLeft(b)) => a < b,
                (UpRight(a), UpRight(b)) => a < b,
                (DownLeft(a), DownLeft(b)) => a < b,
                (DownRight(a), DownRight(b)) => a < b,
                _ => false,
            };
            if is_blocking {
                return is_blocking;
            }
        }
        false
    }
}

fn is_friendly_fire(gm: &GameManager, end: &Position) -> bool {
    let pieces = match gm.turn {
        Player::Black => &gm.blacks,
        Player::White => &gm.whites,
    };
    for piece in pieces {
        if piece.row == end.row && piece.column == end.column {
            return true;
        }
    }
    false
}

fn create_blacks_from_whites(whites: &Vec<Piece>) -> Vec<Piece> {
    let mut blacks = Vec::with_capacity(16);
    for i in whites {
        blacks.push(Piece::new(i.kind, 7 - i.row, i.column))
    }
    blacks
}

fn create_whites() -> Vec<Piece> {
    let mut whites: Vec<Piece> = Vec::with_capacity(16);
    whites.push(Piece::new(Kind::Rook, 0, 0));
    whites.push(Piece::new(Kind::Knight, 0, 1));
    whites.push(Piece::new(Kind::Bishop, 0, 2));
    whites.push(Piece::new(Kind::Queen, 0, 3));
    whites.push(Piece::new(Kind::King, 0, 4));
    whites.push(Piece::new(Kind::Bishop, 0, 5));
    whites.push(Piece::new(Kind::Knight, 0, 6));
    whites.push(Piece::new(Kind::Rook, 0, 7));
    for i in 0..8 {
        whites.push(Piece::new(Kind::Pawn, 1, i));
    }
    whites
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        move_validators::is_pawn_in_start_pos,
        piece::{Piece, Player, Position},
    };

    use super::{GameManager, Kind};

    #[test]
    fn test_piece_at_start() {
        let gm = GameManager::new();
        assert_eq!(Player::White, gm.turn);
        let correct_order: Vec<Kind> = vec![
            Kind::Rook,
            Kind::Knight,
            Kind::Bishop,
            Kind::Queen,
            Kind::King,
            Kind::Bishop,
            Kind::Knight,
            Kind::Rook,
        ];
        let mut positions: HashMap<usize, Kind> = HashMap::new();
        for i in correct_order.iter().enumerate() {
            positions.insert(i.0, *i.1);
        }
        // check correct Pawn position for whites
        for piece in &gm.whites {
            if let Kind::Pawn = piece.kind {
                assert!(is_pawn_in_start_pos(&piece, &Player::White));
                continue;
            }
            match positions.get(&usize::from(piece.column)) {
                None => panic!("white piece {:?} in incorect position", piece),
                Some(k) => {
                    if k != &piece.kind || piece.row != 0 {
                        panic!("white piece {:?} in incorect position", piece)
                    }
                }
            }
        }
        // check correct Pawn position for blacks
        for piece in &gm.blacks {
            if let Kind::Pawn = piece.kind {
                assert!(is_pawn_in_start_pos(piece, &Player::Black,));
                continue;
            }
            match positions.get(&usize::from(piece.column)) {
                None => panic!("black piece {:?} in incorect position", piece),
                Some(k) => {
                    if k != &piece.kind || piece.row != 7 {
                        panic!("Black piece {:?} in incorect position", piece)
                    }
                }
            }
        }
    }

    fn test_move_suggestion() {
        let gm = GameManager::new();
        let piece = Piece::new(Kind::Rook, 2, 2);
        let pieces = gm.blacks.iter().chain(gm.whites.iter());
        let positions: Vec<Position> = pieces
            .filter(|p| !(p.row == piece.row && p.column == piece.column))
            .map(Position::from_piece)
            .collect();
    }

    #[test]
    fn test_is_piece_blocking_forward_white() {
        let targetw = Position::new(4, 4);
        let targetb = Position::new(3, 3);
        let gm = GameManager {
            turn: Player::White,
            whites: vec![Piece::new(Kind::Pawn, targetw.row - 1, targetw.column)],
            blacks: vec![Piece::new(Kind::Pawn, targetb.row - 1, targetb.column)],
        };
        for target in [targetw, targetb] {
            let piece = Piece::new(Kind::Pawn, target.row - 2, target.column);
            if !gm.is_piece_blocking(&piece, &target) {
                panic!("Should be blocking")
            }
        }
    }

    #[test]
    fn test_is_piece_blocking_diag_white() {
        let targetw = Position::new(4, 4);
        let targetb = Position::new(3, 3);
        let gm = GameManager {
            turn: Player::White,
            whites: vec![Piece::new(Kind::Pawn, targetw.row - 1, targetw.column)],
            blacks: vec![Piece::new(Kind::Pawn, targetb.row - 1, targetb.column)],
        };
        for target in [targetw, targetb] {
            let piece = Piece::new(Kind::Pawn, target.row - 2, target.column);
            if !gm.is_piece_blocking(&piece, &target) {
                panic!("Should be blocking")
            }
        }
    }
    #[test]
    fn test_is_piece_blocking_forward_black() {
        let targetw = Position::new(4, 4);
        let targetb = Position::new(3, 3);
        let gm = GameManager {
            turn: Player::Black,
            whites: vec![Piece::new(Kind::Pawn, targetw.row + 1, targetw.column)],
            blacks: vec![Piece::new(Kind::Pawn, targetb.row + 1, targetb.column)],
        };
        for target in [targetw, targetb] {
            let piece = Piece::new(Kind::Pawn, target.row + 2, target.column);
            if !gm.is_piece_blocking(&piece, &target) {
                panic!("Should be blocking")
            }
        }
    }
}
