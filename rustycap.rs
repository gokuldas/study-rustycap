
use pretty_hex::PrettyHex;
use pcaphdr::PcapHdrS;

use std::io::{File, MemReader};
use std::path::Path;
use std::os::args;
use std::io::SeekSet;

mod pretty_hex;
mod pcaphdr;

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
            match PcapHdrS::new(&mut rdr){
                Err(e)  => println!("Failed to read global header: {}", e),
                Ok(hdr) => hdr.display()
            }
        }
    }
}
