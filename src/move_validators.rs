use crate::piece::{Direction, Kind, Piece, Player, Position, MAX_ROW};

pub fn is_pawn_in_start_pos(piece: &Piece, turn: &Player) -> bool {
    match turn {
        Player::Black => piece.row == MAX_ROW - 2,
        Player::White => piece.row == 1,
    }
}

fn is_valid_pawn_move(piece: &Piece, end: &Position, turn: &Player) -> bool {
    use Direction::{Down, DownLeft, DownRight, Up, UpLeft, UpRight};
    use Player::*;

    let start = Position::from_piece(piece);
    let direction = start.get_direction(end);
    let in_start_pos = is_pawn_in_start_pos(piece, &turn);
    if !direction.is_some() {
        return false;
    }
    match (&turn, direction.unwrap()) {
        (White, Up(up)) => {
            if (in_start_pos && up <= 2) || up == 1 {
                return true;
            }
            false
        }
        (White, UpLeft(dir) | UpRight(dir)) => dir == 1,
        (Black, Down(down)) => {
            if (in_start_pos && down <= 2) || down == 1 {
                return true;
            }
            false
        }
        (Black, DownLeft(dir) | DownRight(dir)) => dir == 1,
        _ => false,
    }
}

fn is_valid_bishop_move(piece: &Piece, end: &Position) -> bool {
    use Direction::{DownLeft, DownRight, UpLeft, UpRight};

    let start = Position::from_piece(piece);
    let direction = start.get_direction(end);
    if direction.is_none() {
        return false;
    }
    match direction.unwrap() {
        DownLeft(_) | DownRight(_) | UpLeft(_) | UpRight(_) => true,
        _ => false,
    }
}

fn is_valid_rook_move(piece: &Piece, end: &Position) -> bool {
    use Direction::{Down, Left, Right, Up};
    let start = Position::from_piece(piece);
    let direction = start.get_direction(end);
    if direction.is_none() {
        return false;
    }
    match direction.unwrap() {
        Up(_) | Down(_) | Left(_) | Right(_) => true,
        _ => false,
    }
}

fn is_valid_king_move(piece: &Piece, end: &Position) -> bool {
    use Direction::{Down, DownLeft, DownRight, Left, Right, Up, UpLeft, UpRight};

    let start = Position::from_piece(piece);
    let direction = start.get_direction(end);

    if direction.is_none() {
        return false;
    }
    match direction.unwrap() {
        Down(d) | DownLeft(d) | DownRight(d) | Left(d) | Right(d) | Up(d) | UpLeft(d)
        | UpRight(d) => d == 1,
        _ => false,
    }
}

fn is_valid_queen_move(piece: &Piece, end: &Position) -> bool {
    use Direction::{Down, DownLeft, DownRight, Left, Right, Up, UpLeft, UpRight};

    let start = Position::from_piece(piece);
    let direction = start.get_direction(end);

    if direction.is_none() {
        return false;
    }
    match direction.unwrap() {
        Down(_) | DownLeft(_) | DownRight(_) | Left(_) | Right(_) | Up(_) | UpLeft(_)
        | UpRight(_) => true,
        _ => false,
    }
}

fn is_valid_knight_move(piece: &Piece, end: &Position) -> bool {
    use Direction::Knight;

    let start = Position::from_piece(piece);
    let direction = start.get_direction(end);
    if let Some(Knight(_)) = direction {
        return true;
    }
    false
}

pub fn is_valid_move(piece: &Piece, end: &Position, turn: &Player) -> bool {
    match piece.kind {
        Kind::King => is_valid_king_move(piece, end),
        Kind::Queen => is_valid_queen_move(piece, end),
        Kind::Knight => is_valid_knight_move(piece, end),
        Kind::Bishop => is_valid_bishop_move(piece, end),
        Kind::Rook => is_valid_rook_move(piece, end),
        Kind::Pawn => is_valid_pawn_move(piece, end, turn),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::piece::{Kind, Piece, Player, Position, MAX_COLUMN, MAX_ROW};
    #[test]
    fn test_pawn_move_white() {
        let turn = &Player::White;
        for ci in 1..MAX_COLUMN {
            for ri in 1..MAX_ROW {
                let piece = Piece::new(Kind::Pawn, ri, ci);
                let up = Position::new(piece.row + 1, piece.column);
                let uleft = Position::new(piece.row + 1, piece.column - 1);
                let uright = Position::new(piece.row + 1, piece.column + 1);
                for end in [up, uleft, uright] {
                    if !is_valid_pawn_move(&piece, &end, turn) {
                        panic!("It should be possible to move {:?} to {:?}", &piece, end)
                    }
                }
            }
        }

        // Checking starting position
        let piece = Piece::new(Kind::Pawn, 1, 0);
        let end = Position::new(piece.row + 2, piece.column);
        if !is_valid_pawn_move(&piece, &end, turn) {
            panic!("It should be possible to move {:?} to {:?}", &piece, end)
        }
    }

    #[test]
    fn test_pawn_move_black() {
        let turn = &Player::Black;
        for ci in 1..MAX_COLUMN {
            for ri in 1..MAX_ROW {
                let piece = Piece::new(Kind::Pawn, ri, ci);
                let up = Position::new(piece.row - 1, piece.column);
                let dleft = Position::new(piece.row - 1, piece.column - 1);
                let dright = Position::new(piece.row - 1, piece.column + 1);
                for end in [up, dleft, dright] {
                    if !is_valid_pawn_move(&piece, &end, turn) {
                        panic!("It should be possible to move {:?} to {:?}", &piece, end)
                    }
                }
            }
        }

        // Checking starting position
        let piece = Piece::new(Kind::Pawn, 6, 0);
        let end = Position::new(piece.row - 2, piece.column);
        if !is_valid_pawn_move(&piece, &end, turn) {
            panic!("It should be possible to move {:?} to {:?}", &piece, end)
        }
    }

    #[test]
    fn test_bishop_move() {
        for ci in 1..MAX_COLUMN {
            for ri in 1..MAX_ROW {
                let start = Piece::new(Kind::Bishop, ri, ci);
                let upleft = Position::new(start.row + 1, start.column - 1);
                let upright = Position::new(start.row + 1, start.column + 1);
                let downleft = Position::new(start.row - 1, start.column - 1);
                let downright = Position::new(start.row - 1, start.column + 1);
                for end in [upleft, upright, downleft, downright] {
                    if !is_valid_bishop_move(&start, &end) {
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
                let start = Piece::new(Kind::Rook, ri, ci);
                let left = Position::new(start.row, start.column - 1);
                let right = Position::new(start.row, start.column + 1);
                let forward = Position::new(start.row + 1, start.column);
                let backward = Position::new(start.row + 1, start.column);
                for end in [forward, backward, left, right] {
                    if !is_valid_rook_move(&start, &end) {
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
                let start = Piece::new(Kind::Knight, ri, ci);
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
                    if !is_valid_knight_move(&start, &end) {
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
        // I start from 1 to avoid overflow error, I check for correct move not necessary in a valid range
        for ci in 1..MAX_COLUMN {
            for ri in 1..MAX_ROW {
                let start = Piece::new(Kind::King, ri, ci);
                let up = Position::new(start.row + 1, start.column);
                let down = Position::new(start.row - 1, start.column);

                let right = Position::new(start.row, start.column + 1);
                let left = Position::new(start.row, start.column - 1);

                let uright = Position::new(start.row + 1, start.column + 1);
                let uleft = Position::new(start.row + 1, start.column - 1);

                let dright = Position::new(start.row - 1, start.column + 1);
                let dleft = Position::new(start.row - 1, start.column + 1);

                for end in [up, down, right, left, uright, uleft, dright, dleft] {
                    if !is_valid_king_move(&start, &end) {
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
    fn test_is_pawn_in_start_pos() {
        for ci in 0..MAX_COLUMN {
            for ri in 0..MAX_ROW {
                let white = Piece::new(Kind::Pawn, ri, ci);
                let black = Piece::new(Kind::Pawn, ri, ci);
                if ri == 1 && !is_pawn_in_start_pos(&white, &Player::White) {
                    panic!(
                        "Should be in start position for whites {:?}",
                        Position::from_piece(&white)
                    )
                } else if ri == 6 && !is_pawn_in_start_pos(&black, &Player::Black) {
                    panic!(
                        "Should be in start position for blacks {:?}",
                        Position::from_piece(&black)
                    )
                }
            }
        }
    }
}
