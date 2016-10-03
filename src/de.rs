use std::io;

use serde::de;

use super::error::{Error, ErrorCode, Result};
use super::read::{self, Read};

pub struct Deserializer<R>
    where R: Read
{
    reader: R,
}

impl<R> Deserializer<R>
    where R: Read
{
    pub fn new(reader: R) -> Self {
        Deserializer { reader: reader }
    }

    fn next_char(&mut self) -> Result<u8> {
        match self.reader.next_char() {
            Some(Ok(t)) => Ok(t),
            Some(err_res) => err_res.map_err(From::from),
            None => Err(self.unexpected_eof()),
        }
    }

    fn peek_char(&self) -> Option<u8> {
        self.reader.peek_char()
    }

    fn parse_next<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        const DICT_OPEN: u8 = b'd';
        const LIST_OPEN: u8 = b'l';
        const INT_OPEN: u8 = b'i';

        let ch = try!(self.next_char());
        match ch {
            DICT_OPEN => visitor.visit_map(MapVisitor::new(self)),
            LIST_OPEN => visitor.visit_seq(SeqVisitor::new(self)),
            INT_OPEN => self.parse_int(visitor),
            b'0'...b'9' => self.parse_string(ch, visitor),
            _ => Err(self.unexpected_token(ch)),
        }
    }

    fn parse_string<V>(&mut self, init_len_digit: u8, mut visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        const COLON: u8 = b':';
        if init_len_digit == b'0' {
            let colon = try!(self.next_char());
            if colon != COLON {
                return Err(self.unexpected_token(colon));
            }
            return visitor.visit_str("");
        }

        let len = try!(self.read_digits_to(COLON, Some(init_len_digit))) as usize;
        let mut buf: Vec<u8> = vec![];
        for _ in 0..len {
            let ch = try!(self.next_char());
            buf.push(ch);
        }
        let s = try!(String::from_utf8(buf));
        visitor.visit_string(s)
    }

    fn parse_int<V>(&mut self, mut visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        const END: u8 = b'e';

        let ch = try!(self.next_char());
        let sign = if ch == b'-' { -1 } else { 1 };
        let initnum = if ch == b'-' {
            try!(self.next_char())
        } else {
            ch
        };
        let num: i64 = try!(match initnum {
            b'0' => {
                if sign == -1 {
                    return Err(self.unexpected_token(initnum));
                }
                let end = try!(self.next_char());
                if end != END {
                    return Err(self.unexpected_token(end));
                }
                Ok(0)
            }
            END => Err(self.unexpected_token(END)),
            _ => self.read_digits_to(END, Some(initnum)).map(|n| n * sign),
        });

        visitor.visit_i64(num)
    }

    fn read_digits_to(&mut self, delim: u8, init_digit: Option<u8>) -> Result<i64> {
        const DIGIT_ZERO: i64 = 0x30;
        let mut ch = try!(self.next_char());
        let mut acc: i64 = init_digit.map(|ch| (ch as i64) - DIGIT_ZERO).unwrap_or_default();
        while ch != delim {
            match ch {
                b'0'...b'9' => {
                    acc = 10 * acc + ((ch as i64) - DIGIT_ZERO);
                }
                _ => {
                    return Err(self.unexpected_token(ch));
                }
            }
            ch = try!(self.next_char());
        }

        Ok(acc)
    }

    fn end(&self) -> Result<()> {
        const END: u8 = b'e';
        match self.peek_char() {
            Some(END) | None => Ok(()),
            _ => Err(self.syntax_error(ErrorCode::UnexpectedTrailingChars)),
        }
    }

    fn unexpected_token(&self, ch: u8) -> Error {
        let s = String::from_utf8(vec![ch]).expect("Non-utf8 string encountered!");
        self.syntax_error(ErrorCode::UnexpectedToken(s))
    }

    fn unexpected_eof(&self) -> Error {
        self.syntax_error(ErrorCode::UnexpectedEOF)
    }

    fn syntax_error(&self, code: ErrorCode) -> Error {
        Error::Syntax(code, self.reader.position())
    }
}

impl<R> de::Deserializer for Deserializer<R>
    where R: Read
{
    type Error = Error;

    #[inline]
    fn deserialize<V>(&mut self, visitor: V) -> Result<V::Value>
        where V: de::Visitor
    {
        self.parse_next(visitor)
    }

    forward_to_deserialize! {
        bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 char str string unit option
        seq seq_fixed_size bytes map unit_struct newtype_struct tuple_struct struct struct_field
        tuple enum ignored_any
    }
}

struct MapVisitor<'a, R: Read + 'a> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: Read + 'a> MapVisitor<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        MapVisitor { de: de }
    }
}

impl<'a, R: Read + 'a> de::MapVisitor for MapVisitor<'a, R> {
    type Error = Error;

    fn visit_key<K>(&mut self) -> Result<Option<K>>
        where K: de::Deserialize
    {
        const END: u8 = b'e';
        match self.de.peek_char() {
            Some(END) => Ok(None),
            Some(ch) => {
                match ch {
                    b'0'...b'9' => Ok(Some(try!(de::Deserialize::deserialize(self.de)))),
                    _ => Err(self.de.unexpected_token(ch)),
                }
            }
            _ => Err(self.de.unexpected_eof()),
        }
    }

    fn visit_value<V>(&mut self) -> Result<V>
        where V: de::Deserialize
    {
        Ok(try!(de::Deserialize::deserialize(self.de)))
    }

    fn end(&mut self) -> Result<()> {
        const END: u8 = b'e';
        match try!(self.de.next_char()) {
            END => Ok(()),
            ch => Err(self.de.unexpected_token(ch)),
        }
    }

    fn missing_field<V>(&mut self, field: &'static str) -> Result<V>
        where V: de::Deserialize
    {
        use std;

        struct MissingFieldDeserializer(&'static str);

        impl de::Deserializer for MissingFieldDeserializer {
            type Error = de::value::Error;

            fn deserialize<V>(&mut self, _visitor: V) -> std::result::Result<V::Value, Self::Error>
                where V: de::Visitor
            {
                let &mut MissingFieldDeserializer(field) = self;
                Err(de::value::Error::MissingField(field))
            }

            fn deserialize_option<V>(&mut self,
                                     mut visitor: V)
                                     -> std::result::Result<V::Value, Self::Error>
                where V: de::Visitor
            {
                visitor.visit_none()
            }

            forward_to_deserialize! {
                bool usize u8 u16 u32 u64 isize i8 i16 i32 i64 f32 f64 char str
                string unit seq seq_fixed_size bytes map unit_struct
                newtype_struct tuple_struct struct struct_field tuple enum
                ignored_any
            }
        }

        let mut de = MissingFieldDeserializer(field);
        Ok(try!(de::Deserialize::deserialize(&mut de)))
    }
}

struct SeqVisitor<'a, R: Read + 'a> {
    de: &'a mut Deserializer<R>,
}

impl<'a, R: Read + 'a> SeqVisitor<'a, R> {
    fn new(de: &'a mut Deserializer<R>) -> Self {
        SeqVisitor { de: de }
    }
}

impl<'a, R: Read + 'a> de::SeqVisitor for SeqVisitor<'a, R> {
    type Error = Error;

    fn visit<V>(&mut self) -> Result<Option<V>>
        where V: de::Deserialize
    {
        const END: u8 = b'e';
        match self.de.peek_char() {
            Some(END) => Ok(None),
            Some(_) => Ok(Some(try!(de::Deserialize::deserialize(self.de)))),
            None => Err(self.de.unexpected_eof()),
        }
    }

    fn end(&mut self) -> Result<()> {
        const END: u8 = b'e';
        match self.de.peek_char() {
            Some(END) => Ok(()),
            Some(ch) => Err(self.de.unexpected_token(ch)),
            None => Err(self.de.unexpected_eof()),
        }
    }
}

fn from_read<R, T>(read: R) -> Result<T>
    where R: Read,
          T: de::Deserialize
{
    let mut de = Deserializer::new(read);
    let value = try!(de::Deserialize::deserialize(&mut de));
    try!(de.end());
    Ok(value)
}

fn from_iter<I, T>(iter: I) -> Result<T>
    where I: Iterator<Item = io::Result<u8>>,
          T: de::Deserialize
{
    from_read(read::IteratorRead::new(iter))
}

pub fn from_reader<R, T>(reader: R) -> Result<T>
    where R: io::Read,
          T: de::Deserialize
{
    from_iter(reader.bytes())
}

pub fn from_slice<T>(s: &[u8]) -> Result<T>
    where T: de::Deserialize
{
    from_read(read::SliceRead::new(s))
}

pub fn from_string<T>(s: String) -> Result<T>
    where T: de::Deserialize
{
    from_read(read::StringRead::new(&s))
}
