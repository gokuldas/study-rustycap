
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
                    let mut j = format!("{:X}", i);
                    if i < 16 {
                        j = format!("0{:X}", i);
                    }
                    print!("{}", j);

                    self.byte_no += 1;
                    match self.byte_no {
                        n if n % 16 == 0 => {
                            print!("\n{:X}: ", self.byte_no);
                        }
                        n if n % 4 == 0  => print!("  "),
                        _          => print!(" ")
                    }
                }
            }
        }
        println!("\nTotal of {} bytes", self.byte_no);
    }

}

// TODO: Implement function to set length of hex conversion (for line no)
// TODO: Print line number for first row
