//! This is the bencode algorithm in rust.
//!
//! For more info, see https://wiki.theory.org/BitTorrentSpecification
//!
//! # Examples
//!
//! > Coming soon!
//!

extern crate serde;
extern crate itoa;

pub mod error;
pub mod value;
pub mod ser;

pub use ser::{to_writer, to_vec, to_string};
