
use std::io::{MemReader, IoResult};

//PCAP PACKET/RECORD HEADER
pub struct RecHdrS {
    ts_sec   : u32, /* timestamp seconds (unix time) */
    ts_usec  : u32, /* timestamp microseconds */
    incl_len : u32, /* number of octets of packet saved in file */
    orig_len : u32, /* actual length of packet */
    position : u32
}

impl RecHdrS {

    pub fn new(reader: &mut MemReader) -> IoResult<RecHdrS> {
        let pos = try!(reader.tell());
        Ok(RecHdrS{
            position : pos as u32,
            ts_sec   : try!(reader.read_le_u32()),
            ts_usec  : try!(reader.read_le_u32()),
            incl_len : try!(reader.read_le_u32()),
            orig_len : try!(reader.read_le_u32())
        })
    }

    pub fn display(&self) {
        println!("RECORD HEADER DETAILS");
        println!("Position    : {}", self.position);
        println!("Time(unix-s): {}", self.ts_sec);
        println!("Time(us)    : {}", self.ts_usec);
        println!("Data size   : {}", self.incl_len);
        println!("Packet size : {}", self.orig_len);
    }
}

/* TODO:
1. Implement big endian reader after code refactoring
*/
