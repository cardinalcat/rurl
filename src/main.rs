use std::{io, io::Write, env, fs::File};
fn main(){
    let args: Vec<String> = env::args().collect();
    let url = match args.get(i){
        Some(url) => url,
        None => {
            println!("USAGE: rurl url");
            std::process::exit(1);
        },
    }
    let mut file = File::open(format!("tcp:{}:80", url)).unwrap(); // example.com
    file.write(b"GET / HTTP/1.1\nHost: example.com\nConnection: close\n\n").unwrap();

    io::copy(&mut file, &mut io::stdout()).unwrap();
}
