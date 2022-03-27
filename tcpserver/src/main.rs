use std::{net::TcpListener, io::{Read, Write}};

fn main() {
    let connection_listener = 
        TcpListener::bind("127.0.0.1:3000").unwrap();
        println!("Running on port 3000");

        for stream in connection_listener.incoming(){
            let mut stream = stream.unwrap();
            println!("Connection established");

            let mut buffer = [0; 1024];
            stream.read(&mut buffer).unwrap();
            stream.write(&mut buffer).unwrap();
        }
    // * NOTE: run on cli
    // scenario1> $ cargo run -p tcpserver 
}
