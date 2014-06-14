use std::io::{MemReader, IoResult, IoError, MismatchedFileTypeForOperation};
use dump_decoder::custom_read::{leu32, leu16, lei32, beu32, beu16, bei32};

mod custom_read;

enum PcapFileType { NormLE, NormBE, HResLE, HResBE, Invalid, UnInit}

pub struct DumpDecoder {
    // Global header information
    version_major : u16,  /* major version number */
    version_minor : u16,  /* minor version number */
    thiszone      : i32,  /* GMT to local correction */
    sigfigs       : u32,  /* accuracy of timestamps */
    snaplen       : u32,  /* max length of captured packets, in octets */
    network       : u32,  /* data link type */

    // Dump decoder info and states
    dump          : MemReader,
    file_type     : PcapFileType
}

impl DumpDecoder {

    pub fn new(reader: MemReader) -> DumpDecoder {
        DumpDecoder {
            version_major : 0,
            version_minor : 0,
            thiszone      : 0,
            sigfigs       : 0,
            snaplen       : 0,
            network       : 0,
            dump          : reader,
            file_type     : UnInit
        }
    }

    pub fn decode(&mut self) -> IoResult<()>{
        let mut magic = 0;
        try!(leu32(&mut self.dump, &mut magic));
        match magic {
            0xA1B2C3D4 => {
                try!(leu16(&mut self.dump, &mut self.version_major));
                try!(leu16(&mut self.dump, &mut self.version_minor));
                try!(lei32(&mut self.dump, &mut self.thiszone));
                try!(leu32(&mut self.dump, &mut self.sigfigs));
                try!(leu32(&mut self.dump, &mut self.snaplen));
                try!(leu32(&mut self.dump, &mut self.network));
                self.file_type = NormLE;
                Ok(())
            }
            0xD4C3B2A1 => {
                try!(beu16(&mut self.dump, &mut self.version_major));
                try!(beu16(&mut self.dump, &mut self.version_minor));
                try!(bei32(&mut self.dump, &mut self.thiszone));
                try!(beu32(&mut self.dump, &mut self.sigfigs));
                try!(beu32(&mut self.dump, &mut self.snaplen));
                try!(beu32(&mut self.dump, &mut self.network));
                self.file_type = NormBE;
                Ok(())
            }
            _ => Err(IoError{kind: MismatchedFileTypeForOperation,
                             desc: "File decode: Invalid file type",
                             detail: None })
        }
    }

    pub fn display(&self) {
        match self.file_type {
            UnInit  => println!("Data not decoded"),
            Invalid => println!("Invalid data dump"),
            _       => {
                println!("PCAP GLOBAL HEADER DETAILS");
                println!("Major version  : {  }", self.version_major);
                println!("Minor version  : {  }", self.version_minor);
                println!("Time zone(s)   : {  }", self.thiszone);
                println!("Time precision : {  }", self.sigfigs);
                println!("Snap length    : {  }", self.snaplen);
                println!("Link type      : {  }", self.network);
            }
        }
    }

}

/*
TODO:
1. Validate BEnd case
2. Add display for file type and network type
*/
