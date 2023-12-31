use std::cmp;

use crate::piece::{Kind, MoveType, Piece, Position};

const MAX_ROW: u8 = 8;
const MAX_COLUMN: u8 = MAX_ROW;

#[derive(Debug, PartialEq, Eq)]
enum Player {
    White,
    Black,
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

    pub fn move_piece(&mut self, piece: &Piece, pos: Position) -> Result<(), MoveErr> {
        let pieces = match self.turn {
            Player::Black => &self.blacks,

            Player::White => &self.whites,
        };

        let positions: Vec<Position> = pieces
            .iter()
            .filter(|p| !(p.row == piece.row && p.column == piece.column))
            .map(Position::from_piece)
            .collect();
        if let Some(err) = is_valid_move(self, piece, &pos) {
            return Err(err);
        }

        if is_piece_blocking(
            &Position::from_piece(piece),
            &pos,
            &positions,
            piece.kind.into(),
        ) {
            return Err(MoveErr::PieceBlocking);
        }
        Ok(())
    }
}

#[derive(Debug)]
enum MoveErr {
    SamePosition,
    FriendlyFire,
    InvalidMove,
    PieceBlocking,
}

fn is_pawn_in_start_pos(turn: &Player, piece: &Piece) -> bool {
    match turn {
        Player::Black => piece.row == MAX_ROW - 2,
        Player::White => piece.row == 1,
    }
}

fn is_bishop_move(start: &Position, end: &Position) -> Option<u8> {
    let rmax = cmp::max(start.row, end.row);
    let rmin = cmp::min(start.row, end.row);
    let cmax = cmp::max(start.column, end.column);
    let cmin = cmp::min(start.column, end.column);
    if rmax - rmin == cmax - cmin {
        return Some(rmax - rmin);
    }

    return None;
}

fn is_rook_move(start: &Position, end: &Position) -> Option<u8> {
    let rmax = cmp::max(start.row, end.row);
    let rmin = cmp::min(start.row, end.row);
    let cmax = cmp::max(start.column, end.column);
    let cmin = cmp::min(start.column, end.column);
    if cmax == cmin {
        return Some(rmax - rmin);
    } else if rmax == rmin {
        return Some(cmax - cmin);
    }

    None
}

fn is_king_move(start: &Position, end: &Position) -> bool {
    let rmax = cmp::max(start.row, end.row);
    let rmin = cmp::min(start.row, end.row);
    let cmax = cmp::max(start.column, end.column);
    let cmin = cmp::min(start.column, end.column);
    rmax - rmin == 1 || cmax - cmin == 1
}

fn is_queen_move(start: &Position, end: &Position) -> bool {
    is_bishop_move(start, end).is_some() || is_rook_move(start, end).is_some()
}

fn is_knight_move(start: &Position, end: &Position) -> bool {
    let rmax = cmp::max(start.row, end.row);
    let rmin = cmp::min(start.row, end.row);
    let cmax = cmp::max(start.column, end.column);
    let cmin = cmp::min(start.column, end.column);
    if rmax - rmin == 1 && cmax - cmin == 2 {
        return true;
    } else if cmax - cmin == 1 && rmax - rmin == 2 {
        return true;
    }
    false
}

fn is_pawn_move(start: &Position, end: &Position, start_position: bool) -> bool {
    let diag = is_bishop_move(start, end).is_some_and(|e| e == 1);
    let straight = is_rook_move(start, end);
    if start_position {
        return diag || straight.is_some_and(|e| e < 3);
    }
    diag || straight.is_some_and(|e| e == 1)
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

fn is_valid_move(gm: &GameManager, piece: &Piece, end: &Position) -> Option<MoveErr> {
    let start = &Position::from_piece(piece);
    if start == end {
        return Some(MoveErr::SamePosition);
    } else if is_friendly_fire(gm, end) {
        return Some(MoveErr::FriendlyFire);
    }

    let valid = match piece.kind {
        Kind::King => is_king_move(start, end),
        Kind::Queen => is_queen_move(start, end),
        Kind::Knight => is_knight_move(start, end),
        Kind::Bishop => is_bishop_move(start, end).is_some(),
        Kind::Rook => is_rook_move(start, end).is_some(),
        Kind::Pawn => is_pawn_move(start, end, is_pawn_in_start_pos(&gm.turn, piece)),
    };
    if !valid {
        return Some(MoveErr::InvalidMove);
    }

    None
}

fn is_piece_blocking(
    start: &Position,
    end: &Position,
    positions: &[Position],
    move_type: MoveType,
) -> bool {
    let diag = is_bishop_move(start, end).unwrap_or(0);
    let straight = is_rook_move(start, end).unwrap_or(0);
    for pos in positions {
        let is_blocking = match move_type {
            MoveType::Diagonal => is_bishop_move(start, pos).unwrap_or(0) < diag,
            MoveType::Straight => is_rook_move(start, pos).unwrap_or(0) < straight,
            MoveType::StraightAndDiagonal => {
                is_bishop_move(start, pos).unwrap_or(0) < diag
                    && is_rook_move(start, pos).unwrap_or(0) < straight
            }
            MoveType::Knight => is_king_move(start, pos),
        };

        if is_blocking {
            return is_blocking;
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

    use super::Kind;
    use super::*;

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
                assert!(is_pawn_in_start_pos(&Player::White, piece));
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
                assert!(is_pawn_in_start_pos(&Player::Black, piece));
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
    fn test_is_pawn_in_start_pos() {
        for ci in 0..MAX_COLUMN {
            for ri in 0..MAX_ROW {
                let white = Piece::new(Kind::Pawn, ri, ci);
                let black = Piece::new(Kind::Pawn, ri, ci);
                if ri == 1 && !is_pawn_in_start_pos(&Player::White, &white) {
                    panic!(
                        "Should be in start position for whites {:?}",
                        Position::from_piece(&white)
                    )
                } else if ri == 6 && !is_pawn_in_start_pos(&Player::Black, &black) {
                    panic!(
                        "Should be in start position for blacks {:?}",
                        Position::from_piece(&black)
                    )
                }
            }
        }
    }

    #[test]
    fn test_pawn_move_white() {
        for ci in 1..MAX_COLUMN {
            for ri in 1..MAX_ROW {
                let start = Piece::new(Kind::Pawn, ri, ci);
                let up = Position::new(start.row + 1, start.column);
                let uleft = Position::new(start.row + 1, start.column - 1);
                let uright = Position::new(start.row + 1, start.column + 1);
                for end in [up, uleft, uright] {
                    if !is_pawn_move(
                        &Position::from_piece(&start),
                        &end,
                        is_pawn_in_start_pos(&Player::White, &start),
                    ) {
                        panic!("It should be possible to move {:?} to {:?}", &start, end)
                    }
                }
            }
        }
    }

    #[test]
    fn test_bishop_move() {
        for ci in 1..MAX_COLUMN {
            for ri in 1..MAX_ROW {
                let start = Position::new(ri, ci);
                let upleft = Position::new(start.row + 1, start.column - 1);
                let upright = Position::new(start.row + 1, start.column + 1);
                let downleft = Position::new(start.row - 1, start.column - 1);
                let downright = Position::new(start.row - 1, start.column + 1);
                for end in [upleft, upright, downleft, downright] {
                    if let None = is_bishop_move(&start, &end) {
                        panic!("It should be possible to move {:?} to {:?}", &start, end)
                    }
                }
            }
        }
    }

    #[test]
    fn test_rook_move() {
        for ci in 1..MAX_COLUMN {
            for ri in 1..MAX_ROW {
                let start = Position::new(ri, ci);
                let left = Position::new(start.row, start.column - 1);
                let right = Position::new(start.row, start.column + 1);
                let forward = Position::new(start.row + 1, start.column);
                let backward = Position::new(start.row + 1, start.column);
                for end in [forward, backward, left, right] {
                    if let None = is_rook_move(&start, &end) {
                        panic!(
                            "It should be possible to move from {:?} to {:?}",
                            &start, end
                        )
                    }
                }
            }
        }
    }

    #[test]
    fn test_knight_move() {
        // I start from 2 to avoid overflow error, I check for correct move not necessary in a valid range
        for ci in 2..MAX_COLUMN {
            for ri in 2..MAX_ROW {
                let start = Position::new(ri, ci);
                let upleft = Position::new(start.row + 2, start.column - 1); // up -> up -> left
                let upright = Position::new(start.row + 2, start.column + 1); // up -> up -> right
                let downleft = Position::new(start.row - 2, start.column - 1); // down -> down -> left
                let downright = Position::new(start.row - 2, start.column + 1); // down -> down -> right
                let dleft = Position::new(start.row - 1, start.column - 2); // left -> left -> down
                let dright = Position::new(start.row - 1, start.column + 2); // right -> right -> down
                let uleft = Position::new(start.row + 1, start.column - 2); // left -> left -> up
                let uright = Position::new(start.row + 1, start.column + 2); // right -> right -> up
                for end in [
                    upleft, upright, downleft, downright, dleft, dright, uleft, uright,
                ] {
                    if !is_knight_move(&start, &end) {
                        panic!(
                            "It should be possible to move from {:?} to {:?}",
                            &start, end
                        )
                    }
                }
            }
        }
    }

    #[test]
    fn test_king_move() {
        // I start from 2 to avoid overflow error, I check for correct move not necessary in a valid range
        for ci in 1..MAX_COLUMN {
            for ri in 1..MAX_ROW {
                let start = Position::new(ri, ci);
                let up = Position::new(start.row + 1, start.column);
                let down = Position::new(start.row - 1, start.column);

                let right = Position::new(start.row, start.column + 1);
                let left = Position::new(start.row, start.column - 1);

                let uright = Position::new(start.row + 1, start.column + 1);
                let uleft = Position::new(start.row + 1, start.column - 1);

                let dright = Position::new(start.row - 1, start.column + 1);
                let dleft = Position::new(start.row - 1, start.column + 1);

                for end in [up, down, right, left, uright, uleft, dright, dleft] {
                    if !is_king_move(&start, &end) {
                        panic!(
                            "It should be possible to move from {:?} to {:?}",
                            &start, end
                        )
                    }
                }
            }
        }
    }

    #[test]
    fn test_is_piece_blocking_diag() {
        let start = Position::new(4, 4);

        let uleft = Position::new(start.row + 2, start.column - 2);
        let buleft = Position::new(uleft.row - 1, uleft.column + 1);

        let uright = Position::new(start.row + 2, start.column + 2);
        let buright = Position::new(uleft.row - 1, uleft.column - 1);

        let dleft = Position::new(start.row - 2, start.column - 2);
        let bdleft = Position::new(uleft.row + 1, uleft.column + 1);

        let dright = Position::new(start.row - 2, start.column + 2);
        let bdright = Position::new(uleft.row + 1, uleft.column - 1);

        let positions = vec![buleft, buright, bdleft, bdright];

        for end in &[uleft, uright, dleft, dright] {
            if !is_piece_blocking(&start, end, &positions, MoveType::Diagonal) {
                panic!("{:?} should not be able to go to {:?}", end, &start)
            }
        }
    }

    #[test]
    fn test_is_piece_blocking_forward() {
        let start = Position::new(4, 4);

        let forward = Position::new(start.row + 2, start.column);
        let bforward = Position::new(forward.row - 1, forward.column);

        let left = Position::new(start.row, start.column - 2);
        let bleft = Position::new(left.row, left.column + 1);

        let right = Position::new(start.row, start.column + 2);
        let bright = Position::new(right.row, right.column - 1);

        let down = Position::new(start.row - 2, start.column);
        let bdown = Position::new(down.row + 1, left.column);

        let positions = vec![bforward, bleft, bright, bdown];
        for end in &[forward, left, right, down] {
            if !is_piece_blocking(&start, end, &positions, MoveType::Straight) {
                panic!("{:?} should not be able to go to {:?}", end, &start)
            }
        }
    }
}
