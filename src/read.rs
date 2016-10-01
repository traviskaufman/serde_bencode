use std::io;

use serde::iter;

use super::error::{Result};

pub trait Read {
    fn next_char(&mut self) -> Option<Result<u8>>;
    fn peek_char(&self) -> Option<u8>;
    fn position(&self) -> usize;
}

pub struct IteratorRead<I> where I: Iterator<Item = io::Result<u8>> {
    iter: iter::LineColIterator<I>,
    ch: Option<u8>,
}

impl<I> IteratorRead<I> where I: Iterator<Item = io::Result<u8>> {
    pub fn new(raw_iter: I) -> Self {
        IteratorRead {
            iter: iter::LineColIterator::new(raw_iter),
            ch: None
        }
    }
}

impl<I> Read for IteratorRead<I> where I: Iterator<Item = io::Result<u8>> {
    fn next_char(&mut self) -> Option<Result<u8>> {
        match self.iter.next() {
            Some(Ok(t)) => {
                self.ch = Some(t);
                Some(Ok(t))
            },
            Some(err_res) => Some(err_res.map_err(From::from)),
            _ => None,
        }
    }

    fn peek_char(&self) -> Option<u8> {
        self.ch
    }

    fn position(&self) -> usize {
        self.iter.col()
    }
}

pub struct SliceRead<'a> {
    slice: &'a [u8],
    pos: usize
}

impl<'a> SliceRead<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        SliceRead {
            slice: slice,
            pos: 0
        }
    }
}

impl<'a> Read for SliceRead<'a> {
    fn next_char(&mut self) -> Option<Result<u8>> {
        if let Some(ch) = self.peek_char() {
            self.pos += 1;
            Some(Ok(ch))
        } else {
            None
        }
    }

    fn peek_char(&self) -> Option<u8> {
        if self.pos == self.slice.len() {
            return None;
        }
        Some(self.slice[self.pos])
    }

    fn position(&self) -> usize {
        self.pos
    }
}

pub struct StringRead<'a> {
    slice_read: SliceRead<'a>
}

impl<'a> StringRead<'a> {
    pub fn new(s: &'a String) -> Self {
        StringRead {
            slice_read: SliceRead::new(s.as_bytes()),
        }
    }
}

impl<'a> Read for StringRead<'a> {
    fn next_char(&mut self) -> Option<Result<u8>> {
        self.slice_read.next_char()
    }

    fn peek_char(&self) -> Option<u8> {
        self.slice_read.peek_char()
    }

    fn position(&self) -> usize {
        self.slice_read.position()
    }
}
