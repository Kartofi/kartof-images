use std::io::{BufReader, Read};

#[derive(Debug, Clone)]
pub enum ReqType {
    GET = 1,
    POST = 2,
    OTHER = 3,
}
impl From<ReqType> for isize {
    fn from(value: ReqType) -> isize {
        match value {
            ReqType::GET => 1,
            ReqType::POST => 2,
            ReqType::OTHER => 3,
        }
    }
}
pub struct Param {
    pub name: String,
    pub value: String,
}
pub struct Request {
    pub req_type: ReqType,
    pub path: String,
    pub params: Vec<Param>,
}

pub fn format(req: String) -> Request {
    let mut result = Request {
        req_type: ReqType::OTHER,
        path: "/".to_string(),
        params: Vec::new(),
    };
    let parts: Vec<&str> = req.trim().split(" ").collect();
    //Req type
    if parts[0] == "GET" {
        result.req_type = ReqType::GET;
    } else if parts[0] == "POST" {
        result.req_type = ReqType::POST;
    }
    //Path
    let mut path_parts: Vec<&str> = parts[1].split("?").collect();

    if path_parts.len() > 0 {
        result.path = path_parts[0].to_string();

        if path_parts.len() > 1 {
            let params_string: String = path_parts[1].to_string();

            let mut params: Vec<Param> = Vec::new();

            let params_vec: Vec<&str> = params_string.split("&").collect();
            for param in params_vec.iter() {
                let sides: Vec<&str> = param.split("=").collect();
                if sides.len() != 2 {
                    break;
                }
                params.push(Param {
                    name: sides[0].to_string(),
                    value: sides[1].to_string(),
                });
            }
            result.params = params;
        }
    }

    return result;
}

//Forms
pub fn extract_boundary(http_request: &Vec<String>) -> Option<String> {
    for line in http_request {
        if line.starts_with("Content-Type: multipart/form-data") {
            if let Some(boundary_start) = line.find("boundary=") {
                return Some("--".to_owned() + &line[boundary_start + "boundary=".len()..].trim());
            }
        }
    }
    None
}
pub fn extract_length(http_request: &Vec<String>) -> Option<usize> {
    for line in http_request {
        if line.starts_with("Content-Length") {
            let split: Vec<&str> = line.split(": ").collect();

            // Ensure there are enough parts after splitting
            if split.len() >= 2 {
                // Attempt to parse and trim the second part
                if let Ok(parsed_value) = split[1].trim().parse() {
                    return Some(parsed_value);
                } else {
                    return None;
                }
            } else {
                return None;
            }
        }
    }
    None
}
// Extension trait to read until a specific boundary
pub trait ReadUntilBoundary {
    fn read_length(&mut self, buf: &mut Vec<u8>, length: usize) -> std::io::Result<usize>;
}

impl<R: Read> ReadUntilBoundary for BufReader<R> {
    fn read_length(&mut self, buf: &mut Vec<u8>, length: usize) -> std::io::Result<usize> {
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
            if bytes_read >= length {
                break;
            }
        }
        Ok(bytes_read)
    }
}
