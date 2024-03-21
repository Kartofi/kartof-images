use crate::utils::format_request::ReadUntilBoundary;
use crate::utils::format_response::{self, Route};
use crate::utils::manage_uploads;
use crate::utils::{
    format_request::{self, Request},
    format_response::Routes,
};
use std::{
    clone,
    fs::{self, File},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
};
use std::{thread, time};

static MAX_FILE_SIZE: usize = 20_000_000;

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
    let mut buf_reader = BufReader::new(&mut stream);
    let mut http_request: Vec<String> = Vec::new();
    loop {
        let mut line = String::new();
        buf_reader.read_line(&mut line).unwrap();
        if line.trim().is_empty() {
            break;
        }
        http_request.push(line);
    }

    // Assuming the request contains a multipart form data
    let boundary = format_request::extract_boundary(&http_request);
    let length = format_request::extract_length(&http_request);

    if let Some(boundary) = boundary {
        if let Some(length) = length {
            if length > MAX_FILE_SIZE {
                let contents = fs::read_to_string("ui/not_found.html").unwrap();
                send_code(stream, contents, "200 OK".to_string());
                return;
            }
        } else {
            let contents = fs::read_to_string("ui/not_found.html").unwrap();
            send_code(stream, contents, "200 OK".to_string());
            return;
        }
        let id = manage_uploads::get_id();
        // Create a new file to write the uploaded data
        let mut file = File::create(format!("images/{}.png", id)).unwrap();

        // Read until the end of the boundary and write to file
        let mut data = Vec::new();
        buf_reader
            .read_until_boundary(&mut data, length.unwrap())
            .unwrap();
        // Skip over the headers
        let content_start = data
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .unwrap_or(0);
        data.drain(..content_start + 4);
        file.write_all(&data).unwrap();

        let contents = format!("http://localhost:7878/image?id={}", id);
        send_code(stream, contents, "200 OK".to_string());
        return;
    }

    if http_request.is_empty() {
        return;
    }
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
        let contents = fs::read_to_string("ui/not_found.html").unwrap();
        send_code(stream, contents, "200 OK".to_string());
    }
}
// Function to extract boundary from the HTTP request
fn send_code(mut stream: TcpStream, reason: String, code: String) {
    let status_line = "HTTP/1.1 ";

    let length_ = reason.len();

    let response = format!("{status_line}{code}\r\n Content-Length: {length_}\r\n\r\n{reason}");
    stream.write_all(&response.as_bytes()).unwrap();
}
