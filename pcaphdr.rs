
use std::io::{MemReader, IoResult, IoError, MismatchedFileTypeForOperation};

enum PcapFileType { NormLE, NormBE, HResLE, HResBE}

// PCAP Global Header
pub struct PcapHdrS {
    magic_number  : u32,  /* magic number */
    version_major : u16,  /* major version number */
    version_minor : u16,  /* minor version number */
    thiszone      : i32,  /* GMT to local correction */
    sigfigs       : u32,  /* accuracy of timestamps */
    snaplen       : u32,  /* max length of captured packets, in octets */
    network       : u32,  /* data link type */
    file_type     : PcapFileType
}

impl PcapHdrS {

    pub fn new(reader: &mut MemReader) -> IoResult<PcapHdrS>{
        let magic = try!(reader.read_le_u32());
        match magic {
            0xA1B2C3D4 => Ok(PcapHdrS{
                magic_number:  0xA1B2C3D4,
                version_major: try!(reader.read_le_u16()),
                version_minor: try!(reader.read_le_u16()),
                thiszone:      try!(reader.read_le_i32()),
                sigfigs:       try!(reader.read_le_u32()),
                snaplen:       try!(reader.read_le_u32()),
                network:       try!(reader.read_le_u32()),
                file_type:     NormLE
            }),
            0xD4C3B2A1 => Ok(PcapHdrS{
                magic_number:  0xD4C3B2A1,
                version_major: try!(reader.read_be_u16()),
                version_minor: try!(reader.read_be_u16()),
                thiszone:      try!(reader.read_be_i32()),
                sigfigs:       try!(reader.read_be_u32()),
                snaplen:       try!(reader.read_be_u32()),
                network:       try!(reader.read_be_u32()),
                file_type:     NormBE
            }),
            _ => Err(IoError{kind: MismatchedFileTypeForOperation,
                             desc: "Wrong file type",
                             detail: None })
        }
    }

    pub fn display(&self) {
        println!("PCAP GLOBAL HEADER DETAILS");
        println!("Magic number   : {:X}", self.magic_number);
        println!("Major version  : {  }", self.version_major);
        println!("Minor version  : {  }", self.version_minor);
        println!("Time zone(s)   : {  }", self.thiszone);
        println!("Time precision : {  }", self.sigfigs);
        println!("Snap length    : {  }", self.snaplen);
        println!("Link type      : {  }", self.network);
    }

}

/*
TODO:
1. Validate BEnd case
*/
