use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let _http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let status_line = "HTTP/1.1 200 OK";
    let contents: Vec<_> = match fs::read("public/tracks/Electric Prince.mp3") {
        Ok(file) => file,
        Err(err) => panic!("{}", err),
    };

    let length = contents.len();
    let headers =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");

    stream.write_all(headers.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
}