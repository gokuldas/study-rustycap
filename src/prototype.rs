
use std::io::{MemReader, IoResult};

pub enum Endianness { BigEndian, LittleEndian, Unknown }

pub trait Descriptor {
    fn init(&mut self, reader: &mut MemReader, order: Endianness) -> IoResult<()>;
    fn display(&self);
}
