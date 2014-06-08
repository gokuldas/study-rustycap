use std::io::File;
use std::path::Path;

fn main() {
    let path = &Path::new("dump.pcap");
/*  println!("Path   : {} \n
              Status : {}", path.display(), path.exists()); */

    let mut file = File::open(path);

    let buf : &mut [u8] = box [0u8, ..100];
    let res = file.read(buf);
    // Take care of IOError

    pretty_hex(buf);

}

fn pretty_hex(buf: &[u8]) {
    let mut k = 0u8;
    for i in buf.iter() {
        let mut j = format!("{:X}", *i);
        if *i < 16 {
            j = format!("0{:X}", *i);
        }
        print!("{}", j);

        k += 1;
        match k {
            16         => {
                println!("");
                k = 0;
            }
            4 | 8 | 12 => print!("  "),
            _          => print!(" ")
        }
    }
    println!("");
}
