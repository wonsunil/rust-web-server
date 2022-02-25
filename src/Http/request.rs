use crate::Session;
use crate::SessionStorage;

pub struct HttpRequest{
    request_url: String,
    data: String,
    session: Session
}

impl HttpRequest{
    pub fn get_request_url(&self) -> String {
        self.request_url.clone()
    }

    pub fn get_data(&self) -> String {
        self.data.clone()
    }

    pub fn new(url: String) -> HttpRequest {
        HttpRequest{
            request_url: url,
            data: "".into(),
            session: Session::empty()
        }
    }

    pub fn new_with_data(data: String) -> HttpRequest {
        HttpRequest{
            request_url: "".into(),
            data: data,
            session: Session::empty()
        }
    }
}
