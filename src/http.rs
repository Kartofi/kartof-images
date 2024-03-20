use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
};

use crate::utils::format_response::{self, Route};
use crate::utils::{
    format_request::{self, Request},
    format_response::Routes,
};

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Started HTTP Server");

    let mut routes: Routes = Routes { routes: Vec::new() };
    routes.init("./src/routes".to_string());

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream, &routes);
    }
}

fn handle_connection(mut stream: TcpStream, routes: &Routes) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    if http_request.len() == 0 {
        return;
    }
    println!("{}", http_request.join("\n"));
    let req: Request = format_request::format(http_request[0].to_string());

    if let Some(route) = routes.get_file(&req.path) {
        let status_line = "HTTP/1.1 200 OK";

        let mut contents: Vec<u8> = Vec::new();

        let content_type: &String = &route.content_type;

        if route.path != "/image" {
            contents = fs::read(&route.file).unwrap();
        } else {
            if req.params.len() == 0 || req.params[0].name != "id" {
                contents = fs::read("ui/no-image.png").unwrap();
            } else {
                let path = format!("{}{}.png", &route.file, req.params[0].value);
                let img_path = Path::new(&path);
                if img_path.exists() == true {
                    contents = fs::read(img_path).unwrap();
                } else {
                    contents = fs::read("ui/no-image.png").unwrap();
                }
            }
        }

        let length = contents.len();

        let response = format!(
            "{status_line}\r\n{content_type}\r\nContent-Length: {length}\r\n\r\n",
            status_line = status_line,
            content_type = content_type,
            length = length
        );
        // Write the response headers
        stream.write_all(response.as_bytes()).unwrap();

        // Write the PNG content
        stream.write_all(&contents).unwrap();
    } else {
        let status_line = "HTTP/1.1 404 Not Found";

        let contents = fs::read_to_string("ui/not_found.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\n Content-Length: {length}\r\n\r\n{contents}");
        stream.write_all(&response.as_bytes()).unwrap();
    }
}
