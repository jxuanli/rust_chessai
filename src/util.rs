use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::board::{Board, BoardBuilder};
use crate::piece::Piece;
use crate::position::Position;
use crate::{Color, Evaluate, Move};

pub fn format_fen(
    board: &Board,
    halfmove_clock: u8,
    fullmove_number: u8,
) -> Result<String, String> {
    let mut fen: Vec<String> = vec![];

    placement(board, &mut fen);
    active_color(&mut fen, board);
    castling(&mut fen, board);
    en_passant(&mut fen, board);
    half_move(&mut fen, halfmove_clock);
    full_move(&mut fen, fullmove_number);

    Ok(fen.join(""))
}

fn active_color(fen: &mut Vec<String>, board: &Board) {
    fen.push(" ".to_string());
    fen.push(
        match board.get_turn_color() {
            Color::Black => "b",
            Color::White => "w",
        }
        .to_string(),
    );
}

fn placement(board: &Board, fen: &mut Vec<String>) {
    let mut skip: i32 = 0;
    for row in (0..8).rev() {
        for col in 0..8 {
            let piece = board.get_piece(Position::new(row, col));
            match_piece(piece, &mut skip, fen);
        }
        check_skip_and_row(&mut skip, fen, row);
    }
}

fn match_piece(piece: Option<Piece>, skip: &mut i32, fen: &mut Vec<String>) {
    match piece {
        None => { *skip += 1; }
        Some(piece) => {
            if *skip != 0 {
                fen.push(skip.to_string());
                *skip = 0;
            }
            let ch = match piece {
                Piece::King(_, _) => "k",
                Piece::Knight(_, _) => "n",
                Piece::Bishop(_, _) => "b",
                Piece::Queen(_, _) => "q",
                Piece::Pawn(_, _) => "p",
                Piece::Rook(_, _) => "r",
            }
            .to_string();
            fen.push(match piece.get_color() {
                Color::White => ch.to_uppercase(),
                Color::Black => ch,
            });
        }
    };
}

fn check_skip_and_row(skip: &mut i32, fen: &mut Vec<String>, row: i32) {
    if *skip != 0 {
        fen.push(skip.to_string());
        *skip = 0;
    }
    if row != 0 { fen.push("/".to_string()); }
}

fn castling(fen: &mut Vec<String>, board: &Board) {
    fen.push(" ".to_string());
    let mut any_castling_rights = false;
    let white_castling_rights = board.get_castling_rights(Color::White);
    if white_castling_rights.can_kingside_castle() {
        fen.push("K".to_string());
        any_castling_rights = true;
    }
    if white_castling_rights.can_queenside_castle() {
        fen.push("Q".to_string());
        any_castling_rights = true
    }
    let black_castling_rights = board.get_castling_rights(Color::Black);
    if black_castling_rights.can_kingside_castle() {
        fen.push("k".to_string());
        any_castling_rights = true;
    }
    if black_castling_rights.can_queenside_castle() {
        fen.push("q".to_string());
        any_castling_rights = true
    }
    if !any_castling_rights { fen.push("-".to_string()); }
}

fn en_passant(fen: &mut Vec<String>, board: &Board) {
    fen.push(" ".to_string());
    fen.push(match board.get_en_passant() {
        None => "-".to_string(),
        Some(position) => {
            let position = format!("{}", position);
            position
        }
    });
}

fn half_move(fen: &mut Vec<String>, halfmove_clock: u8) {
    fen.push(" ".to_string());
    let halfmove_clock = halfmove_clock.to_string();
    fen.push(halfmove_clock);
}

fn full_move(fen: &mut Vec<String>, fullmove_number: u8) {
    fen.push(" ".to_string());
    let fullmove_number = fullmove_number.to_string();
    fen.push(fullmove_number);
}

pub fn parse_fen(fen: &str) -> Result<Board, String> {
    let mut parts = fen.split_ascii_whitespace();
    // fen has six parts
    let placement = parts.next();
    let active_color = parts.next();
    let castling = parts.next();
    let en_passant = parts.next();
    let _halfmove_clock = parts.next();
    let _fullmove_number = parts.next();
    // make sure all parts present
    if placement.is_none()
        || active_color.is_none()
        || castling.is_none()
        || en_passant.is_none()
        || parts.next().is_some()
    {
        return Err(String::from("wrong number of spaces"));
    }

    let mut builder = BoardBuilder::default();

    // parse placement (from white's perspective)
    let mut row: i32 = 7;
    let mut col: i32 = 0;
    for c in placement.unwrap().chars() {
        if (col > 7 && c != '/') || row < 0 {
            return Err(String::from("too many pieces"));
        }
        match c {
            x if x.is_alphabetic() => {
                let color = match x.is_uppercase() {
                    true => Color::White,
                    false => Color::Black,
                };
                // using as because row/col always in u8 range
                let position = Position::new(row as i32, col as i32);
                let piece = match x.to_ascii_lowercase() {
                    'b' => Piece::Bishop(color, position),
                    'n' => Piece::Knight(color, position),
                    'q' => Piece::Queen(color, position),
                    'k' => Piece::King(color, position),
                    'p' => Piece::Pawn(color, position),
                    'r' => Piece::Rook(color, position),
                    _ => {
                        return Err(String::from("unexpected piece"));
                    }
                };
                builder = builder.piece(piece);
                col += 1;
            }
            x if x.is_numeric() => {
                // skip squares
                let skip = x.to_digit(10).unwrap();
                col += skip as i32;
            }
            '/' => {
                if col != 8 {
                    return Err(String::from("incomplete row"));
                }
                col = 0;
                row -= 1;
            }
            _ => {}
        };
    }
    if col != 8 && row != 0 {
        return Err(String::from("incomplete position"));
    }

    builder = builder.set_turn(match active_color.unwrap() {
        "b" => Color::Black,
        "w" => Color::White,
        _ => {
            return Err(String::from("invalid active color"));
        }
    });

    match castling.unwrap() {
        "-" => {}
        castling => {
            for c in castling.chars() {
                let color = match c.is_uppercase() {
                    true => Color::White,
                    false => Color::Black,
                };
                match c.to_ascii_lowercase() {
                    'k' => {
                        builder = builder.enable_kingside_castle(color);
                    }
                    'q' => {
                        builder = builder.enable_queenside_castle(color);
                    }
                    _ => {
                        return Err(String::from("invalid castling side"));
                    }
                };
            }
        }
    };

    builder = builder.set_en_passant(match en_passant.unwrap() {
        "-" => None,
        some => match Position::pgn(some) {
            Ok(position) => Some(position),
            _ => {
                return Err(String::from("invalid en passant"));
            }
        },
    });

    Ok(builder.build())
}

pub fn parse_san_move(board: &Board, move_str: &str) -> Result<Move, String> {
    if move_str == "0-0" {
        return Ok(Move::KingSideCastle {});
    } else if move_str == "0-0-0" {
        return Ok(Move::QueenSideCastle {});
    }

    let mut chars = move_str.chars();

    // optional pawn promotion
    let mut last = chars.next_back();
    let color = board.get_turn_color();
    let offboard = Position::new(-1, -1);
    let move_promotion = compute_move_promotion(last, color, offboard);
    if move_promotion.is_some() { last = chars.next_back(); }
    let to: String = vec![chars.next_back().unwrap_or(' '), last.unwrap_or(' ')]
        .into_iter()
        .collect();
    let move_to = match compute_move_to(to) {
        Ok(value) => value,
        Err(value) => return value,
    };
    let mut source_column = chars.next();
    let piece = compute_piece(source_column, color, offboard);
    if piece != Piece::Pawn(color, offboard) { source_column = chars.next(); }
    let column = parse_source_column(source_column);
    let mut source_row = source_column;
    if column.is_some() { source_row = chars.next();}
    let row = parse_source_row(source_row);
    let mut candidates = vec![];
    do_legal_moves(board, move_to, piece, column, row, &mut candidates);
    match_candidates(candidates, move_promotion, move_to)
}

fn compute_move_promotion(last: Option<char>, color: Color, offboard: Position) -> Option<Piece> {
    let move_promotion = match last {
        Some('Q') => Some(Piece::Queen(color, offboard)),
        Some('K') => Some(Piece::King(color, offboard)),
        Some('N') => Some(Piece::Knight(color, offboard)),
        Some('B') => Some(Piece::Bishop(color, offboard)),
        Some('R') => Some(Piece::Rook(color, offboard)),
        _ => None,
    };
    move_promotion
}

fn compute_move_to(to: String) -> Result<Position, Result<Move, String>> {
    let move_to = match Position::pgn(&to) {
        Ok(position) => position,
        Err(_) => {
            return Err(Err("invalid to position".to_string()));
        }
    };
    Ok(move_to)
}

fn compute_piece(source_column: Option<char>, color: Color, offboard: Position) -> Piece {
    let piece = match source_column {
        Some('B') => Piece::Bishop(color, offboard),
        Some('K') => Piece::King(color, offboard),
        Some('N') => Piece::Knight(color, offboard),
        Some('Q') => Piece::Queen(color, offboard),
        Some('R') => Piece::Rook(color, offboard),
        _ => Piece::Pawn(color, offboard),
    };
    piece
}

fn parse_source_column(source_column: Option<char>) -> Option<i32> {
    let column = match source_column {
        Some('a') => Some(0),
        Some('b') => Some(1),
        Some('c') => Some(2),
        Some('d') => Some(3),
        Some('e') => Some(4),
        Some('f') => Some(5),
        Some('g') => Some(6),
        Some('h') => Some(7),
        _ => None,
    };
    column
}

fn parse_source_row(source_row: Option<char>) -> Option<i32> {
    let row = match source_row {
        Some('1') => Some(0),
        Some('2') => Some(1),
        Some('3') => Some(2),
        Some('4') => Some(3),
        Some('5') => Some(4),
        Some('6') => Some(5),
        Some('7') => Some(6),
        Some('8') => Some(7),
        _ => None,
    };
    row
}

fn do_legal_moves(board: &Board, move_to: Position, piece: Piece, column: Option<i32>, row: Option<i32>, candidates: &mut Vec<Piece>) {
    for legal_move in board.get_legal_moves() {
        if let Move::Piece(from, to) = legal_move {
            if move_to == to {
                if let Some(board_piece) = board.get_piece(from) {
                    // filter based on type
                    let pos = board_piece.get_pos();
                    if board_piece.get_name() == piece.get_name()
                        && (column.is_none() || column == Some(pos.get_col()))
                        && (row.is_none() || row == Some(pos.get_row()))
                    {
                        candidates.push(board_piece);
                    }
                }
            }
        }
    }
}

fn match_candidates(candidates: Vec<Piece>, move_promotion: Option<Piece>, move_to: Position) -> Result<Move, String> {
    match candidates.len() {
        0 => Err("no matching move".to_string()),
        1 => {
            let move_from = candidates[0].get_pos();
            match move_promotion {
                None => Ok(Move::Piece(move_from, move_to)),
                Some(piece) => Ok(Move::Promotion(move_from, move_to, piece)),
            }
        }
        _ => Err("ambiguous move".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use alloc::string::{String, ToString};

    use super::*;
    use crate::board::*;
    use crate::position::*;
    use crate::{GameResult, Move};

    #[test]
    fn test_fen() {
        let start = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

        let board = Board::default();
        assert_eq!(format_fen(&board, 0, 1).unwrap(), String::from(start));

        let board = parse_fen(start).unwrap();
        assert_eq!(format_fen(&board, 0, 1).unwrap(), String::from(start));

        let board = match board.play_move(Move::Piece(E2, E4)) {
            GameResult::Continuing(board) => board,
            _ => panic!("e4 failed"),
        };
        assert_eq!(
            format_fen(&board, 0, 1).unwrap(),
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1".to_string()
        );

        let board = match board.play_move(Move::Piece(C7, C5)) {
            GameResult::Continuing(board) => board,
            _ => panic!("c5 failed"),
        };
        assert_eq!(
            format_fen(&board, 0, 2).unwrap(),
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq c6 0 2".to_string()
        );

        let board = match board.play_move(Move::Piece(G1, F3)) {
            GameResult::Continuing(board) => board,
            _ => panic!("Nf3 failed"),
        };
        assert_eq!(
            format_fen(&board, 1, 2).unwrap(),
            "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 1 2".to_string()
        )
    }

    #[test]
    fn test_parse_san_move() {
        let mut board = Board::default();

        // try valid moves from starting position
        assert_eq!(
            parse_san_move(&board, "d4").expect("d4"),
            Move::Piece(D2, D4)
        );
        assert_eq!(
            parse_san_move(&board, "Nc3").expect("Nc3"),
            Move::Piece(B1, C3)
        );
        // not valid first move for white
        assert_eq!(
            parse_san_move(&board, "d5").expect_err("d5"),
            "no matching move".to_string()
        );

        // make first move
        board = match board.play_move(Move::Piece(E2, E4)) {
            GameResult::Continuing(board) => board,
            e => panic!("unexpected error: {:?}", e),
        };
        // valid first move for black
        assert_eq!(
            parse_san_move(&board, "d5").expect("d5"),
            Move::Piece(D7, D5)
        );
        // white moves not valid for black
        assert_eq!(
            parse_san_move(&board, "c4").expect_err("c4"),
            "no matching move".to_string()
        );
    }
}