
use pretty_hex::PrettyHex;
use std::io::File;
use std::path::Path;

mod pretty_hex;

fn main() {
    let path = &Path::new("dump.pcap");
/*  println!("Path   : {} \n
              Status : {}", path.display(), path.exists()); */

    let mut file = File::open(path);

    let buf : &mut [u8] = box [0u8, ..100];
    let res = file.read(buf);
    // TODO: Take care of IOError

    let mut hexprint = PrettyHex::new();
    hexprint.display(buf);

}
