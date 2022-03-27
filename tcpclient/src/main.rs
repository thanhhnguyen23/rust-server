use std::net::TcpStream;

fn main() {
    let _stream = 
        TcpStream::connect("localhost:3000").unwrap();
    
    // * NOTE: run on cli
    // scenario1> $ cargo run -p tcpclient
}