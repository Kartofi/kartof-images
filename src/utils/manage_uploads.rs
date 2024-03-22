use std::{
    fs::File,
    io::{BufReader, Write},
    net::TcpStream,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::http::{send_code, MAX_FILE_SIZE};

use super::format_request::{self, ReadUntilBoundary};

pub fn get_id() -> String {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
    return nanos.to_string();
}
pub fn handle_upload(
    mut buf_reader: BufReader<&mut TcpStream>,
    mut stream: TcpStream,
    http_request: Vec<String>,
) {
    // Assuming the request contains a multipart form data
    let boundary = format_request::extract_boundary(&http_request);
    let length = format_request::extract_length(&http_request);

    if let Some(boundary) = boundary {
        if let Some(length) = length {
            if length > MAX_FILE_SIZE {
                return;
            }
        } else {
            return;
        }
        let id = get_id();
        // Create a new file to write the uploaded data
        let mut file = File::create(format!("images/{}.png", id)).unwrap();

        // Read until the end of the boundary and write to file
        let mut data = Vec::new();
        buf_reader.read_length(&mut data, length.unwrap()).unwrap();
        // Skip over the headers
        let content_start = data
            .windows(4)
            .position(|window| window == b"\r\n\r\n")
            .unwrap_or(0);
        data.drain(..content_start + 4);
        file.write_all(&data).unwrap();

        let contents = format!("/image?id={}", id);
        send_code(&stream, contents, "200 OK".to_string());
        return;
    }
}
