use::std::net::TcpStream;
use std::io::{Read, Write};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("localhost:3000").unwrap();
    //hello를 TCP 서버 커넥션에 쓴다
    stream.write("Hello".as_bytes()).unwrap();
    let mut buffer = [0;5];
    stream.read(&mut buffer).unwrap();
    println!(
        "Get response from server:{:?}",
        str::from_utf8(&buffer).unwrap()
    );

}
