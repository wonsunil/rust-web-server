use crate::Session;

pub struct HttpRequest{
    request_url: String,
    response: String,
    session: Session
}

impl HttpRequest{
    pub fn get_request_url(&self) -> String {
        self.request_url
    }
}