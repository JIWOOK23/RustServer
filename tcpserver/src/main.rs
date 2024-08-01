use std::net::TcpListener;
use std::io::{Read,Write};
//TcpStream은 Read 와 Write 트레이트를 구현한다
//std::io 모듈을 포함시켜 Read와 Write트레이트를 가져온다

fn main() {
    //서버 초기화 및 주소 바인딩, 포트는 3000
    let connection_listener = TcpListener::bind(
        "127.0.0.1:3000"
    ).unwrap();
    println!("Running on port 3000");
    //소켓 서버는 유입되는 커넥션을 기다린다
    for stream in connection_listener.incoming(){
        //새로운 커넥션이 유입된다
        //스트림을 뮤터블로 만들어 읽고 쓸 수 있게 한다
        let mut stream = stream.unwrap();
        //Result<TcpStream.Error>은 언랩에 성공하면 TcpStream을 반환하고, 커넥션 오류가 발생하며
        //패닉과 함께 프로그램을 종료한다
        println!("Connection established");
        let mut buffer = [0; 1024];
        //유입되는 스트림으로부터 읽는다
        stream.read(&mut buffer).unwrap();
        //받은 데이터를 같은 커넥션을 통해 클라이언트에게 다시 전송한다
        stream.write(&mut buffer).unwrap();
    }
}