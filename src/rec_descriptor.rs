
use std::io::{MemReader, IoResult, IoError, InvalidInput};
use std::option::Option;
use prototype::{Descriptor, Endianness, LittleEndian, BigEndian, Unknown};

pub struct RecDescriptor {
    // Generic descriptor information (super struct use case)
    pl_begin : u32, /* position of packet payload in the buffer */
    pl_size  : u32, /* number of octets of packet saved in file */
    pl_desc  : Option<Box<Descriptor>>, /* Payload descriptor */

    // Packet header specific information
    ts_sec   : u32, /* timestamp seconds (unix time) */
    ts_usec  : u32, /* timestamp microseconds */
    orig_len : u32  /* actual length of packet */
}

impl RecDescriptor {
    pub fn new() -> RecDescriptor {
        RecDescriptor {
            pl_begin : 0, pl_size : 0, pl_desc : None,
            ts_sec   : 0, ts_usec : 0, orig_len : 0
        }
    }
}

impl Descriptor for RecDescriptor {

    fn init(&mut self, reader: &mut MemReader, order: Endianness) -> IoResult<()> {
        match order {
            LittleEndian => {
                self.ts_sec   = try!(reader.read_le_u32());
                self.ts_usec  = try!(reader.read_le_u32());
                self.pl_size  = try!(reader.read_le_u32());
                self.orig_len = try!(reader.read_le_u32());
                self.pl_begin = try!(reader.tell()) as u32;
            }
            BigEndian => {
                self.ts_sec   = try!(reader.read_le_u32());
                self.ts_usec  = try!(reader.read_le_u32());
                self.pl_size  = try!(reader.read_le_u32());
                self.orig_len = try!(reader.read_le_u32());
                self.pl_begin = try!(reader.tell()) as u32;
            }
            Unknown => return Err(IoError{kind: InvalidInput,
                                          desc: "Record decode: Unknown endianness",
                                          detail: None })
        }
        self.pl_desc  = None;
        Ok(())
    }

    fn display(&self) {
        println!("RECORD HEADER DETAILS");
        println!("Position     : {}", self.pl_begin);
        println!("Time(unix-s) : {}", self.ts_sec);
        println!("Time(us)     : {}", self.ts_usec);
        println!("Payload size : {}", self.pl_size);
        println!("Packet size  : {}", self.orig_len);
    }

    fn get_pl_size(&self) -> i64{
        self.pl_size as i64
    }
}

/* TODO:
1. Implement big endian reader after code refactoring
2. Implement super-struct for all decriptors when inheritance matures
*/
