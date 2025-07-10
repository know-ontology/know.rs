// This is free and unencumbered software released into the public domain.

use std::{
    fmt,
    io::{self, Write},
};

#[derive(Clone, Debug, Default)]
pub struct DisplayConfig {
    pub color: bool,
    pub emojis: bool,
}

#[derive(Debug)]
pub struct DisplayInline<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayOneliner<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayConcise<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayDetailed<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayMarkdown<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayMime<'a, T>(pub &'a T);

#[derive(Debug)]
pub struct DisplayJsonLd<'a, T>(pub &'a T);

pub struct WriteToFormatter<'a, 'b>(&'a mut fmt::Formatter<'b>);

impl<'a, 'b> WriteToFormatter<'a, 'b> {
    pub fn new(formatter: &'a mut fmt::Formatter<'b>) -> Self {
        Self(formatter)
    }
}

impl<'a, 'b> Write for WriteToFormatter<'a, 'b> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let s = std::str::from_utf8(buf)
            .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8"))?;

        self.0
            .write_str(s)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "format error"))?;

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(()) // formatters don't need explicit flushing
    }
}
