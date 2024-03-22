use crate::utils::format_request::{ReadUntilBoundary, ReqType};
use crate::utils::format_response::{self, Route};
use crate::utils::manage_uploads::{self, handle_upload};
use crate::utils::{
    format_request::{self, Request},
    format_response::Routes,
};
use std::borrow::BorrowMut;
use std::{
    clone,
    fs::{self, File},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
};
use std::{thread, time};

pub static MAX_FILE_SIZE: usize = 20_000_000;

pub fn start() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    println!("Started HTTP Server");

    let mut routes: Routes = Routes { routes: Vec::new() };
    routes.init("./src/routes".to_string());

    for stream in listener.incoming() {
        let clone = routes.clone();
        thread::spawn(move || {
            let stream = stream.unwrap();
            handle_connection(stream, &clone);
        });
    }
}

fn handle_connection(mut stream: TcpStream, routes: &Routes) {
    let clone_stream = stream.try_clone().unwrap();
    let mut buf_reader: BufReader<&mut TcpStream> = BufReader::new(&mut stream);
    let mut http_request: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();
        if line.trim().is_empty() {
            break;
        }
        http_request.push(line);
    }

    if http_request.is_empty() {
        return;
    }
    let req: Request = format_request::format(http_request[0].to_string());
    let isize_req: isize = req.req_type.into();

    if let Some(route) = routes.get_file(&req.path, isize_req) {
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
        let post_req: isize = ReqType::POST.into();
        if req.path == "/upload" && isize_req == post_req {
            manage_uploads::handle_upload(buf_reader, clone_stream, http_request);
            return;
        } else {
            let contents = fs::read_to_string("ui/not_found.html").unwrap();
            send_code(&stream, contents, "200 OK".to_string());
        }
    }
}

pub fn send_code(mut stream: &TcpStream, reason: String, code: String) {
    let status_line = "HTTP/1.1 ";

    let length_ = reason.len();

    let response = format!("{status_line}{code}\r\n Content-Length: {length_}\r\n\r\n{reason}");
    stream.write_all(&response.as_bytes()).unwrap();
}
