
use pretty_hex::PrettyHex;
use std::io::File;
use std::path::Path;

mod pretty_hex;

fn main() {
    let mut file = File::open(&Path::new("dump.pcap"));

    match file.read_to_end() {
        Ok(ref buf) => {
            let mut hexprint = PrettyHex::new();
            hexprint.display(buf);
        }
        Err(e)  => fail!("file error: {}", e)
    }

}
