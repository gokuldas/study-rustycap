#![crate_id = "rcap#0.1.0"]

#![desc = "Project RustyCap"]
#![license = "BSD"]
#![comment = "Experimental Rust language project for parsing pcap files"]

#![crate_type = "bin"]

use pretty_hex::PrettyHex;
use dump_decoder::DumpDecoder;

use std::io::{File, MemReader};
use std::path::Path;
use std::os::args;
use std::io::SeekSet;

mod pretty_hex;
mod dump_decoder;
mod rec_descriptor;
mod prototype;

fn main() {
    let argums = args();
    if argums.len() != 2 {
        println!("Command syntax: rustycap filename");
        fail!("Invalid command syntax");
    }

    let mut file = File::open(&Path::new(argums.get(1).as_slice()));
    match file.read_to_end() {
        Err(e)  => fail!("file error: {}", e),
        Ok(buf) => {
            let mut rdr = MemReader::new(buf);
            let mut hexprint = PrettyHex::new();
            hexprint.display(&mut rdr);
            rdr.seek(0, SeekSet);  //Not expecting any failure
            println!("");
            let mut decoder = DumpDecoder::new(rdr);
            match decoder.decode(){
                Err(e) => fail!("Failed to decode dump: {}", e),
                Ok(()) => decoder.display()
            }
        }
    }
}
