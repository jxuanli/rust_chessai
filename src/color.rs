#![no_std]
#[macro_use]
extern crate alloc;
use alloc::{string::{String, ToString}, vec::Vec};

pub const WHITE: Color = Color::White;
pub const BLACK: Color = Color::Black;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Color {
    White,
    Black,
}

impl core::fmt::Display for Color {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), core::fmt::Error> {
        write!(fmt,"{}", 
                match self {
                    Self::White => "White",
                    Self::Black => "Black",
                })
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
