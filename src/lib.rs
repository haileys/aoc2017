use std::io::{self, Read};

/// reads all input on stdin
pub fn read_input() -> String {
    let mut buff = String::new();
    io::stdin().read_to_string(&mut buff)
        .expect("read string from stdin");
    buff
}
