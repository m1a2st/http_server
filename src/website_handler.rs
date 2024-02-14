use super::server::{Handler};
use super::http::{Method, Request, Response, StatusCode};
use std::fs;

pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        return WebsiteHandler { public_path };
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        match fs::canonicalize(format!("{}/{}", self.public_path, file_path)) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory Traversal Attack Attempted: {}", file_path);
                    None
                }
            }
            Err(_) => { None }
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => {
                match request.path() {
                    "/" => { Response::new(StatusCode::Ok, self.read_file("index.html")) }
                    path => match self.read_file(path) {
                        Some(content) => Response::new(StatusCode::Ok, Some(content)),
                        None => Response::new(StatusCode::NotFound, None)
                    }
                }
            }
            _ => return Response::new(StatusCode::NotFound, None)
        }
    }
}