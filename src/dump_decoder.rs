use std::io::{MemReader, SeekCur, IoResult, IoError, MismatchedFileTypeForOperation};
use rec_decoder::RecDecoder;

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
    file_type     : PcapFileType,
    records       : Vec<RecDecoder>
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
            file_type     : UnInit,
            records       : vec![]
        }
    }

    pub fn decode(&mut self) -> IoResult<()>{
        let magic = try!(self.dump.read_le_u32());
        match magic {
            0xA1B2C3D4 => {
                self.version_major = try!(self.dump.read_le_u16());
                self.version_minor = try!(self.dump.read_le_u16());
                self.thiszone      = try!(self.dump.read_le_i32());
                self.sigfigs       = try!(self.dump.read_le_u32());
                self.snaplen       = try!(self.dump.read_le_u32());
                self.network       = try!(self.dump.read_le_u32());
                self.file_type = NormLE;
            }
            0xD4C3B2A1 => {
                self.version_major = try!(self.dump.read_be_u16());
                self.version_minor = try!(self.dump.read_be_u16());
                self.thiszone      = try!(self.dump.read_be_i32());
                self.sigfigs       = try!(self.dump.read_be_u32());
                self.snaplen       = try!(self.dump.read_be_u32());
                self.network       = try!(self.dump.read_be_u32());
                self.file_type = NormBE;
            }
            _ => return Err(IoError{kind: MismatchedFileTypeForOperation,
                                    desc: "File decode: Invalid file type",
                                    detail: None })
        }
        while !self.dump.eof() {
            let i = try!(RecDecoder::new(&mut self.dump));
            match self.dump.seek(i.get_len(), SeekCur) {
                Err(e) => return Err(e),
                Ok(()) => self.records.push(i)
            }
        }
        Ok(())
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
        for i in self.records.iter() {
            println!("");
            i.display();
        }
    }

}

/*
TODO:
1. Validate BEnd case
2. Add display for file type and network type
*/
