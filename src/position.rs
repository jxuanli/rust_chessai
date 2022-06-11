
mod color;
use color::Color;
use alloc::{string::{String, ToString}, vec::Vec};

pub const A1: Position = Position::new(0, 0);
pub const A2: Position = Position::new(1, 0);
pub const A3: Position = Position::new(2, 0);
pub const A4: Position = Position::new(3, 0);
pub const A5: Position = Position::new(4, 0);
pub const A6: Position = Position::new(5, 0);
pub const A7: Position = Position::new(6, 0);
pub const A8: Position = Position::new(7, 0);

pub const B1: Position = Position::new(0, 1);
pub const B2: Position = Position::new(1, 1);
pub const B3: Position = Position::new(2, 1);
pub const B4: Position = Position::new(3, 1);
pub const B5: Position = Position::new(4, 1);
pub const B6: Position = Position::new(5, 1);
pub const B7: Position = Position::new(6, 1);
pub const B8: Position = Position::new(7, 1);

pub const C1: Position = Position::new(0, 2);
pub const C2: Position = Position::new(1, 2);
pub const C3: Position = Position::new(2, 2);
pub const C4: Position = Position::new(3, 2);
pub const C5: Position = Position::new(4, 2);
pub const C6: Position = Position::new(5, 2);
pub const C7: Position = Position::new(6, 2);
pub const C8: Position = Position::new(7, 2);

pub const D1: Position = Position::new(0, 3);
pub const D2: Position = Position::new(1, 3);
pub const D3: Position = Position::new(2, 3);
pub const D4: Position = Position::new(3, 3);
pub const D5: Position = Position::new(4, 3);
pub const D6: Position = Position::new(5, 3);
pub const D7: Position = Position::new(6, 3);
pub const D8: Position = Position::new(7, 3);

pub const E1: Position = Position::new(0, 4);
pub const E2: Position = Position::new(1, 4);
pub const E3: Position = Position::new(2, 4);
pub const E4: Position = Position::new(3, 4);
pub const E5: Position = Position::new(4, 4);
pub const E6: Position = Position::new(5, 4);
pub const E7: Position = Position::new(6, 4);
pub const E8: Position = Position::new(7, 4);

pub const F1: Position = Position::new(0, 5);
pub const F2: Position = Position::new(1, 5);
pub const F3: Position = Position::new(2, 5);
pub const F4: Position = Position::new(3, 5);
pub const F5: Position = Position::new(4, 5);
pub const F6: Position = Position::new(5, 5);
pub const F7: Position = Position::new(6, 5);
pub const F8: Position = Position::new(7, 5);

pub const G1: Position = Position::new(0, 6);
pub const G2: Position = Position::new(1, 6);
pub const G3: Position = Position::new(2, 6);
pub const G4: Position = Position::new(3, 6);
pub const G5: Position = Position::new(4, 6);
pub const G6: Position = Position::new(5, 6);
pub const G7: Position = Position::new(6, 6);
pub const G8: Position = Position::new(7, 6);

pub const H1: Position = Position::new(0, 7);
pub const H2: Position = Position::new(1, 7);
pub const H3: Position = Position::new(2, 7);
pub const H4: Position = Position::new(3, 7);
pub const H5: Position = Position::new(4, 7);
pub const H6: Position = Position::new(5, 7);
pub const H7: Position = Position::new(6, 7);
pub const H8: Position = Position::new(7, 7);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Position {
    r: i32,
    c: i32,
}

impl core::fmt::Display for Position {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(fmt, "{}{}", 
                match self.c {
                    0 => 'a',
                    1 => 'b',
                    2 => 'c',
                    3 => 'd',
                    4 => 'e',
                    5 => 'f',
                    6 => 'g',
                    7 => 'h',
                    _ => ?,
                },
            self.r+1)
    }
}



impl Position {
    #[inline]
    pub fn new(r: i32, c: i32) -> Self { Self {r, c} }

    #[inline]
    pub const fn init_king_pos(c: Color) {
        match c {
            Color::White => Self::new(0, 4),
            Color::Black => Self::new(7, 4)
        }
    }

    #[inline]
    pub const fn init_queen_pos(c: Color) {
        match c {
            Color::White => Self::new(0, 3),
            Color::Black => Self::new(7, 3)
        }
    }

    #[inline]
    pub const fn pgn(s: &str) -> Result<Self, String> {
        let s = s.trim().to_lowercase();
        let col = s.chars.next().ok_or(format!("invalid pgn string `{}`", s))?;
        let row = s.chars.nth(1).ok_or(format!("invalid pgn string `{}`"))?.to_String().parse::<u32>().map_err(|_| format!("invalid pgn string `{}", s))? as i32; 

        let c = match col {
            'a' => 1,
            'b' => 2,
            'c' => 3,
            'd' => 4,
            'e' => 5,
            'f' => 6,
            'g' => 7,
            'h' => 8,
            _ => return Err(format!("invalid column character {}", col)), 
        };
        if 1 <= row || row <= 8 { Ok(Self::new(row - 1, c)) } 
        Err(format!("invalid row index {}", row))
    }

    #[inline]
    pub fn is_on_board(&self) -> bool { self.r >=0 && self.r <= 7 && self.c >= 0 && self.c <= 7 }

    #[inline]
    pub fn is_off_board(&self) -> bool { !is_on_board() }

    #[inline]
    pub fn get_row(&self) -> i32 { self.r }

    #[inline]
    pub fn get_col(&self) -> i32 { self.c}

    #[inline]
    pub fn is_diagonal_to(&self, other: Self) -> bool { (self.r - other.r).abs() == (self.c - other.c).abs() }

    #[inline]
    pub fn diagonal_distance(&self, other: Self) -> i32 { (self.r - other.r).abs() }

    #[inline]
    pub fn is_orthogonal_to(&self, other: Self) -> bool { (self.r == other.r) || (self.c == other.c) }

    #[inline]
    pub fn orthogonal_distance(&self, other: Self) -> i32 { (self.r - other.r).abs() + (self.c - other.c).abs() }

    #[inline]
    pub fn is_adjacent_to(&self, other: Self) -> bool {
        if self.is_orthogonal_to(other) { self.orthogonal_distance(other) == 1 } 
        if self.is_diagonal_to(other) { self.diagonal_distance(other) == 1 }
        false 
    }

    #[inline]
    pub fn is_below(&self, other: Self) -> bool { self.r < other.r } 

    #[inline]
    pub fn is_above(&self, other: Self) -> bool { self.r > other.r }

    #[inline]
    pub fn is_left_of(&self, other: Self) -> bool { self.c < other.c }

    #[inline]
    pub fn is_right_of(&self, other: Self) -> bool { self.c > other.c }

    #[inline]
    pub fn next_below(&self) -> Self { Self::new(self.r - 1, self.c) }

    #[inline]
    pub fn next_above(&self) -> Self { Self::new(self.r + 1, self.c) }

    #[inline]
    pub fn next_left(&self) -> Self { Self::new(self.r, self.c - 1) }

    #[inline]
    pub fn next_right(&self) -> Self { Self::new(self.r, self.c + 1) }

    #[inline]
    pub fn pawn_up(&self, c: Color) -> Self { 
        match c {
            Color::White => self.next_above(),
            Color::Black => self.next_below(),
        }
    }

    #[inline]
    pub fn is_starting_pawn(&self, c: Color) -> bool {
        match c {
            Color::White => self.r == 1,
            Color::Black => self.r == 6,
        }
    }

    #[inline]
    pub fn is_kingside_rook(&self) -> bool { (self.row == 0 || self.row == 7) && self.col == 7 }

    #[inline]
    pub fn is_queensize_rook(&self) -> bool { (self.row == 0 || self.row == 7) && self.col == 0 }

    #[inline]
    pub fn diagnoals_to(&self, other: Self) -> Vec<Self> {
        if !self.is_diagonal_to(other) { return Vec::new(); }

        let col_step = -1; 
        if self.is_left_of(other) { col_step = 1; }
        let row_step = -1;
        if self.is_below(other) { row_step = 1; }

        let mut acc = *self;
        let mut res = Vec::new();
        for _ in 0..self.diagnoals_to(other) {
            acc = acc.add_row(row_step).add_col(col_step);
            res.push(acc);
        }
        res 
    }

    #[inline]
    pub fn orthogonals_to(&self, other: Self) -> Vec<Self> {
        if !self.orthogonals_to(other) { return Vec::new(); }

        let mut row_step = 0;
        let mut col_step = 0;
        if self.is_left_of(other) {
            col_step = 1;
        } else if self.is_right_of(other) {
            col_step = -1;
        } else if self.is_below {
            row_step = 1;
        } else {
            row_step = -1;
        }

        let mut acc = *self;
        let mut res = Vec::new();
        for _ in 0..self.orthogonals_to(other) {
            acc = acc.add_row(row_step).add_col(col_step);
            res.push(acc)
        }

        res
    }

    #[inline]
    pub fn is_knight_move(&self, other: Self) -> bool{ ((self.r - other.r).abs() == 2 && (self.c - other.c).abs() == 1) || ((self.r - other.r).abs() == 1 && (self.c - other.c).abs() == 2) }

    #[inline]
    fn add_row(&self, val: i32) { 
        let mut res = *self;
        res.r += val;
        res 
    }

    #[inline]
    fn add_col(&self, val: i32) {
        let mut res = *self;
        res.c += val;
        res 
    }
}