pub enum Method{
    Get,
    Post,
    Null
}

impl Method {
    pub fn _name(&mut self) -> &str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            _ => "Null"
        }
    }
}

pub fn get_method(method: &str) -> Method {
    match method {
        "GET" => Method::Get,
        "POST" => Method::Post,
        _ => Method::Null
    }
}