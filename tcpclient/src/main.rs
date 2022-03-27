use std::{net::TcpStream, io::{Write, Read}};

fn main() {
    let mut stream = 
        TcpStream::connect("localhost:3000").unwrap();

        stream.write("Hello".as_bytes()).unwrap();

        let mut buffer = [0; 5];

        stream.read(&mut buffer).unwrap();

        println!(
            "got response from server: {:?}",
            std::str::from_utf8(&buffer).unwrap()
        );
    
    // * NOTE: run on cli
    // scenario1> $ cargo run -p tcpclient
}