use hello::ThreadPool;

use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

struct WebServer {
    listener: TcpListener,
    pool: ThreadPool,
}

impl WebServer {
    // Initialize the web server
    fn build(addr: &str, num_threads: usize) -> Self {
        let listener = TcpListener::bind(addr).unwrap();
        let pool = ThreadPool::new(num_threads);
        WebServer { listener, pool }
    }

    // Start the web server
    fn run(&self) {
        for stream in self.listener.incoming() {
            let stream = stream.unwrap();

            self.pool.execute(|| {
                Self::compare(stream);
            });
        }
    }

    // Handle incoming connections
    fn compare(mut stream: TcpStream) {
        let buf_reader = BufReader::new(&mut stream);
        let request_line = buf_reader.lines().next().unwrap().unwrap();

        let (status_line, filename) = match &request_line[..] {
            "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
            "GET /sleep HTTP/1.1" => {
                thread::sleep(Duration::from_secs(10));
                ("HTTP/1.1 200 OK", "hello.html")
            }
            _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
        };

        let contents = fs::read_to_string(filename).unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}", status_line = status_line, length = length, contents = contents);

        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn main() {
    let server = WebServer::build("127.0.0.1:7878", 4);
    server.run();
}
