use std::io::{stdin, stdout, Read, Write};

fn main() {
    let mut buf = [0u8; 262144];
    loop {
        let n = stdin().read(&mut buf).expect("read failed");
        if n == 0 {
            break;
        }
        stdout().write_all(&buf[..n]).expect("write failed");
    }
}
