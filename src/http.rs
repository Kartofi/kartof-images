use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Started HTTP Server");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    if http_request.len() == 0 {
        return;
    }
    if http_request[0] == "GET / HTTP/1.1" {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read_to_string("ui/hello.html").unwrap();

        let length = contents.len();

        let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let status_line = "HTTP/1.1 200 OK";
        let contents = fs::read("ui/images.jpg").unwrap();

        // Set the content type header to image/png
        let content_type = "Content-Type: image/png";

        let length = contents.len();

        let response = format!(
            "{status_line}\r\n{content_type}\r\nContent-Length: {length}\r\n\r\n",
            status_line = status_line,
            content_type = content_type,
            length = length
        );
        println!("{}",response);
        // Write the response headers
        stream.write_all(response.as_bytes()).unwrap();

        // Write the PNG content
        stream.write_all(&contents).unwrap();
    }
}
