
pub struct PrettyHex {
    byte_no : u8
}


impl PrettyHex {

    pub fn new() -> PrettyHex {
        PrettyHex {byte_no : 0}
    }

    pub fn reset(&mut self) {
        self.byte_no = 0;
    }

    pub fn display(&mut self, buf: &Vec<u8>) {
        for i in buf.iter() {
            let mut j = format!("{:X}", *i);
            if *i < 16 {
                j = format!("0{:X}", *i);
            }
            print!("{}", j);

            self.byte_no += 1;
            match self.byte_no {
                16         => {
                    println!("");
                    self.byte_no = 0;
                }
                4 | 8 | 12 => print!("  "),
                _          => print!(" ")
            }
        }
        println!("");
    }

}
