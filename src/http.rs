use std::{
    clone,
    fs::{self, File},
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    path::Path,
};
use std::{thread, time};

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

    if http_request.is_empty() {
        return;
    }

    // Assuming the request contains a multipart form data
    let boundary = extract_boundary(&http_request);

    if let Some(boundary) = boundary {
        // Create a new file to write the uploaded data
        let mut file = File::create("uploaded_file.png").unwrap();

        // Read until the end of the boundary and write to file
        let mut data = Vec::new();
        buf_reader
            .read_until_boundary(&mut data, boundary.as_bytes())
            .unwrap();
        // Skip over the headers
        let content_start = data
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .unwrap_or(0);
        data.drain(..content_start + 4);
        file.write_all(&data).unwrap();
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
        let status_line = "HTTP/1.1 404 Not Found";

        let contents = fs::read_to_string("ui/not_found.html").unwrap();
        let length = contents.len();

        let response = format!("{status_line}\r\n Content-Length: {length}\r\n\r\n{contents}");
        stream.write_all(&response.as_bytes()).unwrap();
    }
}
// Function to extract boundary from the HTTP request
fn extract_boundary(http_request: &Vec<String>) -> Option<String> {
    for line in http_request {
        if line.starts_with("Content-Type: multipart/form-data") {
            if let Some(boundary_start) = line.find("boundary=") {
                return Some("--".to_owned() + &line[boundary_start + "boundary=".len()..].trim());
            }
        }
    }
    None
}

// Extension trait to read until a specific boundary
trait ReadUntilBoundary {
    fn read_until_boundary(&mut self, buf: &mut Vec<u8>, boundary: &[u8])
        -> std::io::Result<usize>;
}

impl<R: Read> ReadUntilBoundary for BufReader<R> {
    fn read_until_boundary(
        &mut self,
        buf: &mut Vec<u8>,
        boundary: &[u8],
    ) -> std::io::Result<usize> {
        let mut read_buf = [0; 4096];
        let mut bytes_read = 0;
        loop {
            let bytes = self.read(&mut read_buf)?;
            if bytes == 0 {
                break; // End of stream
            }
            buf.extend_from_slice(&read_buf[..bytes]);
            bytes_read += bytes;

            // Check if boundary is found
            if buf.windows(boundary.len()).any(|window| window == boundary) {
                break;
            }
        }
        Ok(bytes_read)
    }
}
