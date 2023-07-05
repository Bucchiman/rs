/*
 * FileName:        count-server
 * Author:          8ucchiman
 * CreatedDate:     2023-07-04 23:46:34
 * LastModified:    2023-01-23 14:15:07 +0900
 * Reference:       8ucchiman.jp
 * Description:     ---
 */


use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    sync::{Arc, Mutex},
    thread,
};

const HOST: &'static str = "127.0.0.1";
const PORT: u32 = 1718;

struct Counter {
    pub count: i32,
}

impl Counter {
    pub fn get(&mut self) -> i32 {
        self.count
    }

    pub fn set(&mut self, i: i32) {
        self.count = i;
    }
}


fn main() {
    let listener = TcpListener::bind(format!("{HOST}:{PORT}")).unwrap();
    let counter = Arc::new(Mutex::new(Counter {count: 0}));

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let c = Arc::clone(&counter);
        thread::spawn(move || {
            handle_connection(stream, c);
        });
    }
}

fn handle_connection(mut stream: TcpStream, counter: Arc<Mutex<Counter>>) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, content) = match &request_line[..] {
        "GET / HTTP/1.1" => {
            let mut c = counter.lock().unwrap();
            let count = c.get() + 1;
            println!("{}", count);
            c.set(count);

            ("HTTP/1.1 200 OK", format!("Hello, world. {}", count))
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "Not Found".to_string()),
    };
    let length = content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{content}");
    stream.write_all(response.as_bytes()).unwrap();
}
