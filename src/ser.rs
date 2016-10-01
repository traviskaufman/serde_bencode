use std::io;

use itoa;
use serde::de::Type;
use serde::ser;

use super::error::{Error, ErrorCode, Result};

macro_rules! bencode_int {
    ($w:expr, $i:expr) => {{
        let r: Result<()> = write!($w, "i").map_err(From::from);
        try!(r);
        try!(itoa::write($w, $i));
        write!($w, "e").map_err(From::from)
    }};
}

pub struct Serializer<W> {
    writer: W,
    formatter: Formatter,
}

/// TODO!!! Lexocographically-ordered dictionaries. Yuck. Use BTreeMap
impl<W> Serializer<W> where W: io::Write {
    #[inline]
    pub fn new(writer: W, formatter: Formatter) -> Self {
        Serializer {
            writer: writer,
            formatter: formatter
        }
    }

    pub fn into_inner(self) -> W {
        self.writer
    }
}

impl<W> ser::Serializer for Serializer<W> where W: io::Write {
    type Error = Error;
    type TupleState = State;
    type SeqState = State;
    type TupleStructState = State;
    type TupleVariantState = State;
    type MapState = State;
    type StructState = State;
    type StructVariantState = State;

    #[inline]
    fn serialize_bool(&mut self, _: bool) -> Result<()> {
        Err(Error::Ser(ErrorCode::UnsupportedType(Type::Bool)))
    }

    #[inline]
    fn serialize_isize(&mut self, v: isize) -> Result<()> {
        bencode_int!(&mut self.writer, v)
    }

    #[inline]
    fn serialize_i8(&mut self, v: i8) -> Result<()> {
        bencode_int!(&mut self.writer, v)
    }

    #[inline]
    fn serialize_i16(&mut self, v: i16) -> Result<()> {
        bencode_int!(&mut self.writer, v)
    }

    #[inline]
    fn serialize_i32(&mut self, v: i32) -> Result<()> {
        bencode_int!(&mut self.writer, v)
    }

    #[inline]
    fn serialize_i64(&mut self, v: i64) -> Result<()> {
        bencode_int!(&mut self.writer, v)
    }

    #[inline]
    fn serialize_usize(&mut self, v: usize) -> Result<()> {
        bencode_int!(&mut self.writer, v)
    }

    #[inline]
    fn serialize_u8(&mut self, v: u8) -> Result<()> {
        bencode_int!(&mut self.writer, v)
    }

    #[inline]
    fn serialize_u16(&mut self, v: u16) -> Result<()> {
        bencode_int!(&mut self.writer, v)
    }

    #[inline]
    fn serialize_u32(&mut self, v: u32) -> Result<()> {
        bencode_int!(&mut self.writer, v)
    }

    #[inline]
    fn serialize_u64(&mut self, v: u64) -> Result<()> {
        if v > i64::max_value() as u64 {
            return Err(Error::Ser(ErrorCode::NumberOutOfRange(v)));
        }
        bencode_int!(&mut self.writer, v)
    }

    #[inline]
    fn serialize_f32(&mut self, v: f32) -> Result<()> {
        bencode_int!(&mut self.writer, v as i64)
    }

    #[inline]
    fn serialize_f64(&mut self, v: f64) -> Result<()> {
        bencode_int!(&mut self.writer, v as i64)
    }

    #[inline]
    fn serialize_char(&mut self, v: char) -> Result<()> {
        self.formatter.string(&mut self.writer, &char::to_string(&v))
    }

    #[inline]
    fn serialize_str(&mut self, v: &str) -> Result<()> {
        self.formatter.string(&mut self.writer, v)
    }

    #[inline]
    fn serialize_bytes(&mut self, v: &[u8]) -> Result<()> {
        let mut state = try!(self.serialize_seq(Some(v.len())));
        for byte in v {
            try!(self.serialize_seq_elt(&mut state, byte));
        }
        self.serialize_seq_end(state)
    }

    #[inline]
    fn serialize_unit(&mut self) -> Result<()> {
        Ok(())
    }

    #[inline]
    fn serialize_unit_struct(&mut self, _name: &'static str) -> Result<()> {
        self.serialize_map(Some(0)).map(|_| ())
    }

    #[inline]
    fn serialize_unit_variant(&mut self, _name: &'static str, _variant_index: usize, _variant: &'static str) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_newtype_struct<T: ser::Serialize>(&mut self, _name: &'static str, value: T) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_newtype_variant<T: ser::Serialize>(
            &mut self, _name: &'static str, _variant_index: usize, variant: &'static str, value: T) -> Result<()> {
        try!(self.formatter.dict_open(&mut self.writer));
        try!(self.serialize_str(variant));
        try!(value.serialize(self));
        self.formatter.dict_close(&mut self.writer)
    }

    #[inline]
    fn serialize_none(&mut self) -> Result<()> {
        self.serialize_unit()
    }

    #[inline]
    fn serialize_some<T: ser::Serialize>(&mut self, value: T) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq(&mut self, len: Option<usize>) -> Result<State> {
        try!(self.formatter.list_open(&mut self.writer));
        if len == Some(0) {
            try!(self.formatter.list_close(&mut self.writer));
            return Ok(State::Empty);
        }
        Ok(State::First)
    }

    #[inline]
    fn serialize_seq_elt<T: ser::Serialize>(&mut self, state: &mut State, value: T) -> Result<()> {
        *state = State::Rest;
        value.serialize(self)
    }

    #[inline]
    fn serialize_seq_end(&mut self, state: State) -> Result<()> {
        match state {
            State::Empty => Ok(()),
            _ => self.formatter.list_close(&mut self.writer),
        }
    }

    #[inline]
    fn serialize_seq_fixed_size(&mut self, size: usize) -> Result<State> {
        self.serialize_seq(Some(size))
    }

    #[inline]
    fn serialize_tuple(&mut self, size: usize) -> Result<State> {
        self.serialize_seq(Some(size))
    }

    #[inline]
    fn serialize_tuple_elt<T: ser::Serialize>(&mut self, state: &mut State, value: T) -> Result<()> {
        self.serialize_seq_elt(state, value)
    }

    #[inline]
    fn serialize_tuple_end(&mut self, state: State) -> Result<()> {
        self.serialize_seq_end(state)
    }

    #[inline]
    fn serialize_tuple_struct(&mut self, _name: &'static str, size: usize) -> Result<State> {
        self.serialize_seq(Some(size))
    }

    #[inline]
    fn serialize_tuple_struct_elt<T: ser::Serialize>(&mut self, state: &mut State, value: T) -> Result<()> {
        self.serialize_seq_elt(state, value)
    }

    #[inline]
    fn serialize_tuple_struct_end(&mut self, state: State) -> Result<()> {
        self.serialize_seq_end(state)
    }

    #[inline]
    fn serialize_tuple_variant(
        &mut self,
        _name: &'static str,
        _variant_index: usize,
        variant: &'static str,
        len: usize
    ) -> Result<State> {
        try!(self.formatter.dict_open(&mut self.writer));
        try!(self.serialize_str(variant));
        self.serialize_seq(Some(len))
    }

    #[inline]
    fn serialize_tuple_variant_elt<T: ser::Serialize>(
        &mut self,
        state: &mut State,
        value: T
    ) -> Result<()> {
        self.serialize_seq_elt(state, value)
    }

    #[inline]
    fn serialize_tuple_variant_end(&mut self, state: State) -> Result<()> {
        try!(self.serialize_seq_end(state));
        self.formatter.dict_close(&mut self.writer)
    }

    #[inline]
    fn serialize_map(&mut self, len: Option<usize>) -> Result<State> {
        if len == Some(0) {
            try!(self.formatter.dict_open(&mut self.writer));
            try!(self.formatter.dict_close(&mut self.writer));
            Ok(State::Empty)
        } else {
            try!(self.formatter.dict_open(&mut self.writer));
            Ok(State::First)
        }
    }

    #[inline]
    fn serialize_map_key<T: ser::Serialize>(
        &mut self,
        state: &mut State,
        key: T
    ) -> Result<()> {
        *state = State::Rest;

        // FIXME: Copy over MapKeySerializer?
        key.serialize(self)
    }

    #[inline]
    fn serialize_map_value<T: ser::Serialize>(
        &mut self,
        _: &mut State,
        value: T
    ) -> Result<()> {
        value.serialize(self)
    }

    #[inline]
    fn serialize_map_end(&mut self, state: State) -> Result<()> {
        match state {
            State::Empty => Ok(()),
            _ => self.formatter.dict_close(&mut self.writer),
        }
    }

    #[inline]
    fn serialize_struct(
        &mut self,
        _name: &'static str,
        len: usize
    ) -> Result<State> {
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_elt<V: ser::Serialize>(
        &mut self,
        state: &mut State,
        key: &'static str,
        value: V
    ) -> Result<()> {
        try!(self.serialize_map_key(state, key));
        self.serialize_map_value(state, value)
    }

    #[inline]
    fn serialize_struct_end(&mut self, state: State) -> Result<()> {
        self.serialize_map_end(state)
    }

    #[inline]
    fn serialize_struct_variant(
        &mut self,
        _name: &'static str,
        _variant_index: usize,
        variant: &'static str,
        len: usize
    ) -> Result<State> {
        try!(self.formatter.dict_open(&mut self.writer));
        try!(self.serialize_str(variant));
        self.serialize_map(Some(len))
    }

    #[inline]
    fn serialize_struct_variant_elt<V: ser::Serialize>(
        &mut self,
        state: &mut State,
        key: &'static str,
        value: V
    ) -> Result<()> {
        self.serialize_struct_elt(state, key, value)
    }

    #[inline]
    fn serialize_struct_variant_end(&mut self, state: State) -> Result<()> {
        try!(self.serialize_struct_end(state));
        self.formatter.dict_close(&mut self.writer)
    }
}

#[doc(hidden)]
#[derive(Eq, PartialEq)]
pub enum State {
    Empty,
    First,
    Rest
}

#[derive(Clone, Debug)]
pub struct Formatter;

impl Formatter {
    pub fn string<W>(&self, w: &mut W, s: &str) -> Result<()> where W: io::Write {
        write!(w, "{}:{}", s.len(), s).map_err(From::from)
    }

    pub fn dict_open<W>(&self, w: &mut W) -> Result<()> where W: io::Write {
        write!(w, "d").map_err(From::from)
    }

    pub fn dict_close<W>(&self, w: &mut W) -> Result<()> where W: io::Write {
        write!(w, "e").map_err(From::from)
    }

    pub fn list_open<W>(&self, w: &mut W) -> Result<()> where W: io::Write {
        write!(w, "l").map_err(From::from)
    }

    pub fn list_close<W>(&self, w: &mut W) -> Result<()> where W: io::Write {
        write!(w, "e").map_err(From::from)
    }
}

pub fn to_writer<W: ?Sized + io::Write, T: ser::Serialize>(writer: &mut W, value: &T) -> Result<()> {
    let mut ser = Serializer::new(writer, Formatter);
    try!(value.serialize(&mut ser));
    Ok(())
}

pub fn to_vec<T: ser::Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut writer = Vec::with_capacity(128);
    try!(to_writer(&mut writer, value));
    Ok(writer)
}

pub fn to_string<T: ser::Serialize>(value: &T) -> Result<String> {
    let vec = try!(to_vec(value));
    String::from_utf8(vec).map_err(From::from)
}
