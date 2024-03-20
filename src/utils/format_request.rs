enum ReqType {
    GET = 1,
    POST = 2,
    OTHER = 3,
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
