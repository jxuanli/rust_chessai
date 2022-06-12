#![no_std]
#[macro_use]
extern crate alloc;
use alloc::{
    string::{String, ToString},
    vec::Vec,
};

use core::convert::TryFrom;

mod board;
pub use board::{Board, BoardBuilder};

mod game;
pub use game::{Game, GameAction, GameError, GameOver};

mod square;
pub use square::{Square, EMPTY_SQUARE};

mod piece;
pub use piece::Piece;

mod position;
pub use position::*;

mod util;
pub use util::*;

pub const WHITE: Color = Color::White;
pub const BLACK: Color = Color::Black;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum GameResult {
    Continuing(Board),
    Victory(Color),
    Stalemate,
    IllegalMove(Move),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

impl core::fmt::Display for Color {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(
            f,
            "{}",
            match self {
                Self::White => "White",
                Self::Black => "Black",
            }
        )
    }
}

impl core::ops::Not for Color {
    type Output = Self;
    fn not(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Move {
    QueenSideCastle,
    KingSideCastle,
    Piece(Position, Position),
    Promotion(Position, Position, Piece),
    Resign,
}

impl TryFrom<String> for Move {
    type Error = String;

    fn try_from(repr: String) -> Result<Self, Self::Error> {
        let repr = repr.trim().to_string();

        Ok(match repr.as_str() {
            "resign" | "resigns" => Self::Resign,
            "queenside castle" | "castle queenside" | "O-O-O" | "0-0-0" | "o-o-o" => {
                Self::QueenSideCastle
            }
            "kingside castle" | "castle kingside" | "O-O" | "0-0" | "o-o" => Self::KingSideCastle,
            other => {
                let words = other.split_whitespace().collect::<Vec<&str>>();

                if words.len() == 1 && words[0].len() == 4 {
                    Self::Piece(
                        Position::pgn(&words[0][..2])?,
                        Position::pgn(&words[0][2..4])?,
                    )
                } else if words.len() == 2 {
                    Self::Piece(Position::pgn(words[0])?, Position::pgn(words[1])?)
                } else if words.len() == 3 && words[1] == "to" {
                    Self::Piece(Position::pgn(words[0])?, Position::pgn(words[2])?)
                } else if words.len() == 4 && words[1] == "to" {
                    let piece = Piece::try_from(words[3])?;
                    if piece.is_king() || piece.is_pawn() {
                        return Err(String::from("invalid promotion"));
                    }
                    Self::Promotion(Position::pgn(words[0])?, Position::pgn(words[2])?, piece)
                } else {
                    return Err(format!("invalid move format `{}`", other));
                }
            }
        })
    }
}

impl Move {
    pub fn parse(repr: String) -> Result<Self, String> {
        Self::try_from(repr)
    }
}

impl core::fmt::Display for Move {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            Move::Piece(from, to) => write!(f, "{} to {}", from, to),
            Move::Promotion(from, to, piece) => {
                write!(f, "{} to {} {}", from, to, piece.get_name())
            }
            Move::KingSideCastle => write!(f, "O-O"),
            Move::QueenSideCastle => write!(f, "O-O-O"),
            Move::Resign => write!(f, "Resign"),
        }
    }
}

pub trait Evaluate: Sized {
    fn value_for(&self, color: Color) -> f64;

    fn get_current_player_color(&self) -> Color;

    fn get_legal_moves(&self) -> Vec<Move>;

    fn apply_eval_move(&self, m: Move) -> Self;

    fn get_best_next_move(&self, depth: i32) -> (Move, u64, f64) {
        let legal_moves = self.get_legal_moves();
        let mut best_move_value = -999999.0;
        let mut best_move = Move::Resign;

        let color = self.get_current_player_color();

        let mut board_count = 0;
        for m in &legal_moves {
            let child_board_value = self.apply_eval_move(*m).minimax(
                depth,
                -1000000.0,
                1000000.0,
                false,
                color,
                &mut board_count,
            );
            if child_board_value >= best_move_value {
                best_move = *m;
                best_move_value = child_board_value;
            }
        }

        (best_move, board_count, best_move_value)
    }

    fn get_worst_next_move(&self, depth: i32) -> (Move, u64, f64) {
        let legal_moves = self.get_legal_moves();
        let mut best_move_value = -999999.0;
        let mut best_move = Move::Resign;

        let color = self.get_current_player_color();

        let mut board_count = 0;
        for m in &legal_moves {
            let child_board_value = self.apply_eval_move(*m).minimax(
                depth,
                -1000000.0,
                1000000.0,
                true,
                !color,
                &mut board_count,
            );

            if child_board_value >= best_move_value {
                best_move = *m;
                best_move_value = child_board_value;
            }
        }

        (best_move, board_count, best_move_value)
    }

    fn minimax(
        &self,
        depth: i32,
        mut alpha: f64,
        mut beta: f64,
        is_maximizing: bool,
        getting_move_for: Color,
        board_count: &mut u64,
    ) -> f64 {
        *board_count += 1;

        if depth == 0 {
            return self.value_for(getting_move_for);
        }

        let legal_moves = self.get_legal_moves();
        let mut best_move_value;

        if is_maximizing {
            best_move_value = -999999.0;

            for m in &legal_moves {
                let child_board_value = self.apply_eval_move(*m).minimax(
                    depth - 1,
                    alpha,
                    beta,
                    !is_maximizing,
                    getting_move_for,
                    board_count,
                );

                if child_board_value > best_move_value {
                    best_move_value = child_board_value;
                }

                if best_move_value > alpha {
                    alpha = best_move_value
                }

                if beta <= alpha {
                    return best_move_value;
                }
            }
        } else {
            best_move_value = 999999.0;

            for m in &legal_moves {
                let child_board_value = self.apply_eval_move(*m).minimax(
                    depth - 1,
                    alpha,
                    beta,
                    !is_maximizing,
                    getting_move_for,
                    board_count,
                );
                if child_board_value < best_move_value {
                    best_move_value = child_board_value;
                }

                if best_move_value < beta {
                    beta = best_move_value
                }

                if beta <= alpha {
                    return best_move_value;
                }
            }
        }

        best_move_value
    }
}