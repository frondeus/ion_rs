#![feature(slice_patterns, convert)]
use std::collections::BTreeMap;
extern crate rustc_serialize;

macro_rules! parse_next {
    ($row:expr, $err:expr) => ({
        match $row.next() {
            Some(v) => match v.parse() {
                Ok(v) => v,
                Err(_) => return Err($err)
            },
            None => return Err($err)
        }
    })
}

mod ion;
mod parser;
mod validator;
mod writer;
pub use parser::{ Parser, ParserError };
pub use validator::{ Validator, ValidationError };
pub use writer::Writer;

pub type Dictionary = BTreeMap<String, Value>;
pub use ion::{ decode, decode_from_vec, Decoder, Ion, IonError, FromIon, Section, Value };
pub type Row = Vec<Value>;
