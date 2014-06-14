
use std::io::{MemReader, IoResult};

pub fn leu32(reader: &mut MemReader, var: &mut u32) -> IoResult<()> {
    match reader.read_le_u32() {
        Ok(i)  => {
            *var = i;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn lei32(reader: &mut MemReader, var: &mut i32) -> IoResult<()> {
    match reader.read_le_i32() {
        Ok(i)  => {
            *var = i;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn leu16(reader: &mut MemReader, var: &mut u16) -> IoResult<()> {
    match reader.read_le_u16() {
        Ok(i)  => {
            *var = i;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn beu32(reader: &mut MemReader, var: &mut u32) -> IoResult<()> {
    match reader.read_be_u32() {
        Ok(i)  => {
            *var = i;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn bei32(reader: &mut MemReader, var: &mut i32) -> IoResult<()> {
    match reader.read_be_i32() {
        Ok(i)  => {
            *var = i;
            Ok(())
        }
        Err(e) => Err(e)
    }
}

pub fn beu16(reader: &mut MemReader, var: &mut u16) -> IoResult<()> {
    match reader.read_be_u16() {
        Ok(i)  => {
            *var = i;
            Ok(())
        }
        Err(e) => Err(e)
    }
}
