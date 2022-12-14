use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("localhost:7878").unwrap();

    for stream in listener.incoming() {
        let _stream = stream.unwrap();
        println!("Connection established!");
    }
}
