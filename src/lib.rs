//! This is the bencode algorithm in rust.
//!
//! For more info, see https://wiki.theory.org/BitTorrentSpecification
//!
//! # Examples
//!
//! > Coming soon!
//!

#[macro_use]
extern crate serde;
extern crate itoa;

pub mod error;
pub mod value;
pub mod read;
pub mod ser;
pub mod de;

pub use ser::{to_writer, to_vec, to_string};
pub use de::{from_reader, from_slice, from_string};
