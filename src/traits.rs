use crate::Content;
use std::collections::HashMap;
use std::io::{Read, Write};

pub trait Reader {
    fn read_all(&self) -> Result<HashMap<String, Content>, crate::error::Error>;
    fn get_reader(&self, src: &str) -> Box<dyn Read>;
}

pub trait Writer {
    fn get_writer(&self, permalink: &str) -> Box<dyn Write>;
}
