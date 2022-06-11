#![no_std]
#[macro_use]
extern crate alloc;
use alloc::{string::{String, ToString}, vec::Vec};
use core::convert::TryFrom;

mod piece;
pub use piece::Piece;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Move {
    QueenSideCastle,
    KingSideCastle,
}

