/*
 * FileName:        simple_server
 * Author:          8ucchiman
 * CreatedDate:     2023-07-04 22:59:01
 * LastModified:    2023-01-23 14:15:07 +0900
 * Reference:       8ucchiman.jp
 * Description:     ---
 */


use std::error::Error;
use std:: {
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread
};

const HOST: &'static str = "127.0.0.1";

const PORT: u32 = 1718;


fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind(format!("{HOST}:{PORT}")).unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
    Ok(())
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "Hello, world."),
        _ => ("HTTP/1.1 404 NOT FOUND", "Not Found"),
    };

    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}

