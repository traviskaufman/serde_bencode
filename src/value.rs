//! Value types for Bencode. Bencode is kind of like a python subset. It supports ASCII strings,
//! 64-bit floating-point numbers, lists, and dictionaries.

use std::collections::HashMap;

use serde::de::{Deserialize, Deserializer};

/// Value represents a Bencode value type. BEncode has 4 types: Strings, Integers, Lists, and
/// Dicts. Each is represented here! Strings will be converted to ascii. All numbers will be
/// converted to i64s.
pub enum Value {
    /// Represents a string
    ByteString(String),

    /// Represents a number
    Int(i64),

    /// Represents a list
    List(Vec<Value>),

    /// Represents a dictionary. Note that while some people use [BTreeMaps](https://github.com/rust-lang-nursery/rustc-serialize/issues/56)
    /// for this, I doubt bencode keys will have to be sorted so we'll stick with a HashMap.
    Dict(HashMap<String, Value>),
}
