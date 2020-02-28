use std::{io, io::Write, env, fs::File};
fn main(){
    let mut file = File::open("tcp:93.184.216.34:80").unwrap(); // example.com
    file.write(b"GET / HTTP/1.1\nHost: example.com\nConnection: close\n\n").unwrap();

    io::copy(&mut file, &mut io::stdout()).unwrap();
}
