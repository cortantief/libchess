use crate::{
    move_validators::is_valid_move,
    piece::{Kind, Piece, Player, Position, MAX_COLUMN, MAX_ROW},
};

#[derive(Debug)]
pub enum MoveErr {
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
        let _positions: Vec<Position> = pieces
            .filter(|p| !(p.row == piece.row && p.column == piece.column))
            .map(Position::from_piece)
            .collect();

        if piece.row == end.row && piece.column == end.column {
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
        let pieces = match self.turn {
            Player::Black => &mut self.blacks,
            Player::White => &mut self.whites,
        };
        for p in pieces {
            if p.row != piece.row && p.column == piece.column {
                continue;
            }
            p.column = pos.column;
            p.row = pos.row;
            return Ok(());
        }
        Err(MoveErr::InvalidMove)
    }

    pub fn move_suggestion(&self, piece: &Piece) -> Vec<Position> {
        let mut positions = vec![];
        for ci in 0..MAX_COLUMN {
            for ri in 0..MAX_ROW {
                let pos = Position::new(ri, ci);
                if let None = self.is_valid_move(piece, &pos) {
                    positions.push(pos);
                }
            }
            if piece.kind == Kind::Pawn {
                let mut tmp: Vec<Position> = vec![];
                for pos in positions {
                    if pos.column == piece.column {
                        tmp.push(pos);
                        continue;
                    }
                    let pieces = match self.turn {
                        Player::Black => &self.whites,
                        Player::White => &self.blacks,
                    };
                    for p in pieces {
                        if p.row == pos.row && p.column == pos.column {
                            tmp.push(pos);
                            break;
                        }
                    }
                }
                positions = tmp;
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

    #[test]
    fn test_move_suggestion_pawn() {
        let gm = GameManager::new();
        let piece = Piece::new(Kind::Pawn, 2, 2);
        let expected_pos = Position::new(piece.row + 1, piece.column);
        let suggestion = gm.move_suggestion(&piece);
        if !(suggestion.len() == 1 && expected_pos == suggestion[0]) {
            panic!("Expected position not met {:?}", suggestion)
        }
    }

    #[test]
    fn test_move_suggestion_pawn_white_at_start() {
        let gm = GameManager::new();
        let piece = Piece::new(Kind::Pawn, 1, 2);
        let expected_pos = [
            Position::new(piece.row + 1, piece.column),
            Position::new(piece.row + 2, piece.column),
        ];
        let suggestion = gm.move_suggestion(&piece);
        if expected_pos.len() != suggestion.len() {
            panic!(
                "Expected number of position {} not met {}",
                suggestion.len(),
                expected_pos.len()
            )
        }
        for e in expected_pos {
            let mut valid = false;
            for s in &suggestion {
                if &e == s {
                    valid = true;
                    break;
                }
            }
            if !valid {
                panic!("Expected position {:?} not met", e)
            }
        }
    }

    #[test]
    fn test_move_suggestion_rook() {
        let piece = Piece::new(Kind::Rook, 4, 4);
        let mut targets = vec![];

        targets.push(Position::new(piece.row - 2, piece.column));
        targets.push(Position::new(piece.row, piece.column - 2));
        targets.push(Position::new(piece.row + 2, piece.column));
        targets.push(Position::new(piece.row, piece.column + 2));

        let gm = GameManager {
            turn: Player::White,
            whites: vec![],
            blacks: targets
                .iter()
                .map(|p| Piece::new(Kind::Pawn, p.row, p.column))
                .collect(),
        };

        let mut expected_pos: Vec<Position> = vec![];
        expected_pos.push(Position::new(piece.row - 1, piece.column));
        expected_pos.push(Position::new(piece.row, piece.column - 1));
        expected_pos.push(Position::new(piece.row + 1, piece.column));
        expected_pos.push(Position::new(piece.row, piece.column + 1));
        for target in &targets {
            expected_pos.push(target.clone());
        }
        let suggestion = gm.move_suggestion(&piece);
        if expected_pos.len() != suggestion.len() {
            panic!(
                "Amount of suggestion not the same as expected, {} got {}",
                expected_pos.len(),
                suggestion.len()
            )
        }
        for e in expected_pos {
            let mut valid = false;
            for s in &suggestion {
                if &e == s {
                    valid = true;
                    break;
                }
            }
            if !valid {
                panic!("Not expected target found in suggestion")
            }
        }
    }

    #[test]
    fn test_move_suggestion_bishop() {
        let piece = Piece::new(Kind::Bishop, 4, 4);
        let mut targets = vec![];

        targets.push(Position::new(piece.row - 2, piece.column - 2));
        targets.push(Position::new(piece.row - 2, piece.column + 2));
        targets.push(Position::new(piece.row + 2, piece.column + 2));
        targets.push(Position::new(piece.row + 2, piece.column - 2));

        let gm = GameManager {
            turn: Player::White,
            whites: vec![],
            blacks: targets
                .iter()
                .map(|p| Piece::new(Kind::Pawn, p.row, p.column))
                .collect(),
        };

        let mut expected_pos: Vec<Position> = vec![];
        expected_pos.push(Position::new(piece.row - 1, piece.column - 1));
        expected_pos.push(Position::new(piece.row - 1, piece.column + 1));
        expected_pos.push(Position::new(piece.row + 1, piece.column + 1));
        expected_pos.push(Position::new(piece.row + 1, piece.column - 1));
        for target in &targets {
            expected_pos.push(target.clone());
        }
        let suggestion = gm.move_suggestion(&piece);
        if expected_pos.len() != suggestion.len() {
            panic!(
                "Amount of suggestion not the same as expected, {} got {}",
                expected_pos.len(),
                suggestion.len()
            )
        }
        for e in expected_pos {
            let mut valid = false;
            for s in &suggestion {
                if &e == s {
                    valid = true;
                    break;
                }
            }
            if !valid {
                panic!("Not expected target found in suggestion")
            }
        }
    }

    #[test]
    fn test_move_suggestion_pawn_black_at_start() {
        let mut gm = GameManager::new();
        gm.turn = Player::Black;
        let piece = Piece::new(Kind::Pawn, 6, 2);
        let expected_pos = [
            Position::new(piece.row - 1, piece.column),
            Position::new(piece.row - 2, piece.column),
        ];
        let suggestion = gm.move_suggestion(&piece);
        if expected_pos.len() != suggestion.len() {
            panic!(
                "Expected number of position {} not met {}",
                suggestion.len(),
                expected_pos.len()
            )
        }
        for e in expected_pos {
            let mut valid = false;
            for s in &suggestion {
                if &e == s {
                    valid = true;
                    break;
                }
            }
            if !valid {
                panic!("Expected position {:?} not met", e)
            }
        }
    }
    #[test]
    fn test_move_suggestion_pawn_white_at_start_with_diag() {
        let piece = Piece::new(Kind::Pawn, 1, 2);
        let uleft = Piece::new(Kind::Pawn, piece.row + 1, piece.column - 1);
        let uright = Piece::new(Kind::Pawn, piece.row + 1, piece.column + 1);
        let gm = GameManager {
            turn: Player::White,
            whites: vec![],
            blacks: vec![uleft.clone(), uright.clone()],
        };

        let expected_pos = [
            Position::new(piece.row + 1, piece.column),
            Position::new(piece.row + 2, piece.column),
            Position::from_piece(&uleft),
            Position::from_piece(&uright),
        ];
        let suggestion = gm.move_suggestion(&piece);
        if expected_pos.len() != suggestion.len() {
            panic!(
                "Expected number of position {} not met {}",
                suggestion.len(),
                expected_pos.len()
            )
        }
        for e in expected_pos {
            let mut valid = false;
            for s in &suggestion {
                if &e == s {
                    valid = true;
                    break;
                }
            }
            if !valid {
                panic!("Expected position {:?} not met", e)
            }
        }
    }

    #[test]
    fn test_move_suggestion_pawn_black_at_start_with_diag() {
        let piece = Piece::new(Kind::Pawn, 6, 2);
        let uleft = Piece::new(Kind::Pawn, piece.row - 1, piece.column - 1);
        let uright = Piece::new(Kind::Pawn, piece.row - 1, piece.column + 1);
        let gm = GameManager {
            turn: Player::Black,
            whites: vec![uleft.clone(), uright.clone()],
            blacks: vec![],
        };

        let expected_pos = [
            Position::new(piece.row - 1, piece.column),
            Position::new(piece.row - 2, piece.column),
            Position::from_piece(&uleft),
            Position::from_piece(&uright),
        ];
        let suggestion = gm.move_suggestion(&piece);
        if expected_pos.len() != suggestion.len() {
            panic!(
                "Expected number of position {} not met {}",
                suggestion.len(),
                expected_pos.len()
            )
        }
        for e in expected_pos {
            let mut valid = false;
            for s in &suggestion {
                if &e == s {
                    valid = true;
                    break;
                }
            }
            if !valid {
                panic!("Expected position {:?} not met", e)
            }
        }
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
            whites: vec![
                Piece::new(Kind::Pawn, targetw.row - 1, targetw.column - 1),
                Piece::new(Kind::Pawn, targetw.row - 1, targetw.column + 1),
            ],
            blacks: vec![
                Piece::new(Kind::Pawn, targetb.row - 1, targetb.column - 1),
                Piece::new(Kind::Pawn, targetb.row - 1, targetb.column + 1),
            ],
        };
        for target in [&targetw, &targetb] {
            let piece = Piece::new(Kind::Pawn, target.row - 2, target.column - 2);
            if !gm.is_piece_blocking(&piece, &target) {
                panic!("Should be blocking")
            }
        }

        for target in [targetw, targetb] {
            let piece = Piece::new(Kind::Pawn, target.row - 2, target.column + 2);
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

    #[test]
    fn test_is_piece_blocking_diag_black() {
        let targetw = Position::new(4, 4);
        let targetb = Position::new(3, 3);
        let gm = GameManager {
            turn: Player::Black,
            whites: vec![
                Piece::new(Kind::Pawn, targetw.row + 1, targetw.column - 1),
                Piece::new(Kind::Pawn, targetw.row + 1, targetw.column + 1),
            ],
            blacks: vec![
                Piece::new(Kind::Pawn, targetb.row + 1, targetb.column - 1),
                Piece::new(Kind::Pawn, targetb.row + 1, targetb.column + 1),
            ],
        };
        for target in [&targetw, &targetb] {
            let piece = Piece::new(Kind::Pawn, target.row + 2, target.column - 2);
            if !gm.is_piece_blocking(&piece, &target) {
                panic!("Should be blocking")
            }
        }

        for target in [targetw, targetb] {
            let piece = Piece::new(Kind::Pawn, target.row + 2, target.column + 2);
            if !gm.is_piece_blocking(&piece, &target) {
                panic!("Should be blocking")
            }
        }
    }
}
