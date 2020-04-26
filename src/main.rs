use std::io;
use std::io::prelude::*;
use std::char;

const LINE_SIZE: usize = 16;

fn main() {
    let stdin = io::stdin();
    let data = stdin.lock();
    parse(data);
}

pub fn parse<T: BufRead + Sized>(mut data: T) {
    let mut buffer= [0; LINE_SIZE];
    let mut pos: usize = 0;

    while let Ok(_) = data.read_exact(&mut buffer) {
        print!("[{:016x}] ", pos);
        let mut content = vec!();

        for i in 0..LINE_SIZE {
            print!("{:02x} ", buffer[i]);
            if (i + 1) % 8 == 0 && i != 0 {
                print!(" ");
            }
        }

        for _ in 0..LINE_SIZE {
            content.push(String::from("."));
        }

        for x in String::from_utf8_lossy(&buffer).match_indices(char::is_alphanumeric) {
            if x.0 >= LINE_SIZE {
                for _ in content.len()..=x.0 {
                    content.push(String::from("."));
                }
            }
            content[x.0] = String::from(x.1);
        }
        println!(" |{}|", content.join(""));
        pos += LINE_SIZE;
    }
}
