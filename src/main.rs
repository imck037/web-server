use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2020").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_client(stream);
    }
}

#[allow(unused)]
fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    if request_line == "GET / HTTP/1.1" {
        let contents = fs::read_to_string("index.html").unwrap();
        let lenth = contents.len();
        let status_line = "HTTP/1.1 200 OK";
        let response = format!("{status_line}\r\nContent-Lenth: {lenth}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let contents = fs::read_to_string("404.html").unwrap();
        let lenth = contents.len();
        let status_line = "HTTP/1.1 404 NOT FOUND";
        let response = format!("{status_line}\r\nContent-Lenth: {lenth}\r\n\r\n{contents}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}
