use super::server::{Handler};
use super::http::{Method, Request, Response, StatusCode};

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        return WebsiteHandler { public_path };
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => {
                match request.path() {
                    "/" => { Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())) }
                    "/hello" => { Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())) }
                    _ => { Response::new(StatusCode::NotFound, None) }
                }
            }
            _ => return Response::new(StatusCode::NotFound, None)
        }
    }
}