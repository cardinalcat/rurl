use std::io::{self, stderr, Read, Write};
use std::time::Duration;

//use argparse::parser::ArgumentParser;

use hyper::status::StatusCode;
use hyper::Client;
use hyper::net::HttpsConnector;
use hyper_rustls::TlsClient;
use hyper::error::Error as HyperError;
use hyper::header::ContentLength;

use pbr::{ProgressBar, Units};

pub fn download(remote_path: &str) -> io::Result<()> {
    let mut stderr = stderr();

    writeln!(stderr, "* Requesting {}", remote_path)?;

    let mut client = Client::with_connector(HttpsConnector::new(TlsClient::new()));
    client.set_read_timeout(Some(Duration::new(5, 0)));
    client.set_write_timeout(Some(Duration::new(5, 0)));
    let mut response = match client.get(remote_path).send() {
        Ok(response) => response,
        Err(HyperError::Io(err)) => return Err(err),
        Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err.to_string()))
    };

    match response.status {
        StatusCode::Ok => {
            let mut count = 0;
            let length = response.headers.get::<ContentLength>().map_or(0, |h| h.0 as usize);

            //let mut file = File::create(&local_path)?;
            let mut pb = ProgressBar::new(length as u64);
            pb.set_units(Units::Bytes);
            loop {
                let mut buf = [0; 8192];
                let res = response.read(&mut buf)?;
                if res == 0 {
                    break;
                }
                count += std::io::stdout().write(&buf[.. res])?;
                pb.set(count as u64);
            }
            writeln!(stderr).unwrap();

            //file.sync_all()?;

            Ok(())
        },
        _ => {
            writeln!(stderr, "* Failure {}", response.status).unwrap();

            Err(io::Error::new(io::ErrorKind::NotFound, format!("{} not found", remote_path)))
        }
    }
}
fn main(){
    //let mut parser = ArgumentParser::new();
    //parser.add_option("")
    let args: Vec<String> = std::env::args().skip(1).collect();
    if let Some(arg) = args.get(0){
        if arg.contains("https://"){
            download(arg.as_str()).unwrap();
        }else{
            download(format!("{}{}", "https://", arg).as_str()).unwrap();
        }
    }else{
        println!("USAGE: rurl url");
    }
}
