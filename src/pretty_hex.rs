
use std::io::MemReader;

pub struct PrettyHex {
    byte_no : uint
}


impl PrettyHex {

    pub fn new() -> PrettyHex {
        PrettyHex {byte_no : 0}
    }

    pub fn reset(&mut self) {
        self.byte_no = 0;
    }

    pub fn display(&mut self, reader: &mut MemReader) {
        while ! reader.eof() {
            match reader.read_u8() {
                Err(e) => fail!("Memory read error: {}", e),
                Ok(i)  => {
                    match self.byte_no {
                        n if n % 16 == 0 => {
                            print!("\n{:0>4X}: ", self.byte_no);
                        }
                        n if n % 4 == 0  => print!("  "),
                        _          => print!(" ")
                    }
                    print!("{:0>2X}", i);
                    self.byte_no += 1;
                }
            }
        }
        println!("\nTotal of {} bytes", self.byte_no);
    }

}

// TODO: Print line number for first row
