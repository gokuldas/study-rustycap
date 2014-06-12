
use pretty_hex::PrettyHex;
use std::io::File;
use std::path::Path;
use std::os::args;

mod pretty_hex;

fn main() {
    let argums = args();
    if argums.len() != 2 {
        println!("Command syntax: rustycap filename");
        fail!("Invalid command syntax");
    }

    let mut file = File::open(&Path::new(argums.get(1).as_slice()));
    match file.read_to_end() {
        Ok(ref buf) => {
            let mut hexprint = PrettyHex::new();
            hexprint.display(buf);
        }
        Err(e)  => fail!("file error: {}", e)
    }

}
