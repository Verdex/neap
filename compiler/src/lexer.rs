
use super::data;

pub fn lex(input : &str) {
    let mut ci = input.char_indices();
    loop {
        match ci.next() {
            Some((i, c)) => {
                println!("{} {}", i, c);
            },
            None => { break }
        }
    }
}

