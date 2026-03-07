use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};
use web_server::Threadpool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:2020").unwrap();
    let pool = Threadpool::new(5);

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_client(stream);
        });
    }
}

#[allow(unused)]
fn handle_client(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(50));
            ("HTTP/1.1 200 OK", "index.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };
    let contents = fs::read_to_string(filename).unwrap();
    let lenth = contents.len();
    let response = format!("{status_line}\r\nContent-Lenth: {lenth}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
