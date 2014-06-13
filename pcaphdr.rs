
use std::io::{MemReader, IoResult};

// PCAP Global Header
pub struct PcapHdrS {
    magic_number  : u32,  /* magic number */
    version_major : u16,  /* major version number */
    version_minor : u16,  /* minor version number */
    thiszone      : i32,  /* GMT to local correction */
    sigfigs       : u32,  /* accuracy of timestamps */
    snaplen       : u32,  /* max length of captured packets, in octets */
    network       : u32   /* data link type */
}

impl PcapHdrS {

    pub fn new(reader: &mut MemReader) -> IoResult<PcapHdrS>{
        Ok(PcapHdrS{
            magic_number:  try!(reader.read_le_u32()),
            version_major: try!(reader.read_le_u16()),
            version_minor: try!(reader.read_le_u16()),
            thiszone:      try!(reader.read_le_i32()),
            sigfigs:       try!(reader.read_le_u32()),
            snaplen:       try!(reader.read_le_u32()),
            network:       try!(reader.read_le_u32())
        })
    }

    pub fn display(&self) {
        println!("PCAP GLOBAL HEADER DETAILS");
        println!("Magic number   : {:X}", self.magic_number);
        println!("Major version  : {:X}", self.version_major);
        println!("Minor version  : {:X}", self.version_minor);
        println!("Time zone      : {:X}", self.thiszone);
        println!("Time precision : {:X}", self.sigfigs);
        println!("Snap length    : {:X}", self.snaplen);
        println!("Link type      : {:X}", self.network);
    }

}

/*
TODO:
1. Handle BEnd & LEnd cases separately
*/
