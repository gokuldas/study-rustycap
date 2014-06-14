
use std::io::{MemReader, IoResult};

pub struct RecDecoder {
    // Packet header information
    ts_sec   : u32, /* timestamp seconds (unix time) */
    ts_usec  : u32, /* timestamp microseconds */
    incl_len : u32, /* number of octets of packet saved in file */
    orig_len : u32, /* actual length of packet */

    data_pos : u32  /* position of packet data in the buffer */
}

impl RecDecoder {

    pub fn new(reader: &mut MemReader) -> IoResult<RecDecoder> {
        Ok(RecDecoder{
            ts_sec   : try!(reader.read_le_u32()),
            ts_usec  : try!(reader.read_le_u32()),
            incl_len : try!(reader.read_le_u32()),
            orig_len : try!(reader.read_le_u32()),
            data_pos : try!(reader.tell()) as u32

        })
    }

    pub fn display(&self) {
        println!("RECORD HEADER DETAILS");
        println!("Position    : {}", self.data_pos);
        println!("Time(unix-s): {}", self.ts_sec);
        println!("Time(us)    : {}", self.ts_usec);
        println!("Data size   : {}", self.incl_len);
        println!("Packet size : {}", self.orig_len);
    }

    pub fn get_len(&self) -> i64{
        self.incl_len as i64
    }
}

/* TODO:
1. Implement big endian reader after code refactoring
*/
