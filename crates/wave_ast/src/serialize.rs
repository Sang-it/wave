use serde::{ser::Serializer, Serialize};
use std::fmt;

use crate::ast::Program;

pub struct EcmaFormatter;

/// Serialize f64 with `ryu_js`
impl serde_json::ser::Formatter for EcmaFormatter {
    fn write_f64<W>(&mut self, writer: &mut W, value: f64) -> std::io::Result<()>
    where
        W: ?Sized + std::io::Write,
    {
        let mut buffer = ryu_js::Buffer::new();
        let s = buffer.format(value);
        writer.write_all(s.as_bytes())
    }
}

impl<'a> Program<'a> {
    /// # Panics
    pub fn to_json(&self) -> String {
        let buf = std::vec::Vec::new();
        let mut ser = serde_json::Serializer::with_formatter(buf, crate::serialize::EcmaFormatter);
        self.serialize(&mut ser).unwrap();
        String::from_utf8(ser.into_inner()).unwrap()
    }
}

pub fn serialize_bigint<T, S>(value: &T, s: S) -> Result<S::Ok, S::Error>
where
    T: fmt::Display,
    S: serde::Serializer,
{
    s.collect_str(&format_args!("{value}n"))
}
