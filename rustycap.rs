
use pretty_hex::PrettyHex;
use std::io::{File, MemReader};
use std::path::Path;
use std::os::args;

mod pretty_hex;

// PCAP Global Header structure
struct PcapHdrS {
    magic_number  : u32,  /* magic number */
    version_major : u16,  /* major version number */
    version_minor : u16,  /* minor version number */
    thiszone      : i32,  /* GMT to local correction */
    sigfigs       : u32,  /* accuracy of timestamps */
    snaplen       : u32,  /* max length of captured packets, in octets */
    network       : u32   /* data link type */
}

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
        }
    }
}

/*impl PcapHdrS {

    fn decode(buf: &Vec<u8>) {
        if buf.len() < 24 {
            fail!("Insufficient global header data");
        } */
