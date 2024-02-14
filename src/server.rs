use std::io::{Read, Write};
use std::net::TcpListener;
use crate::http::{ParseError, Request, Response, StatusCode};


pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self {
            addr,
        }
    }

    pub fn run(self) {
        println!("Server is running on {}", self.addr);

        let listener = TcpListener::bind(&self.addr).unwrap();

        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    let mut buffer = [0; 1024];
                    match stream.read(&mut buffer) {
                        Ok(_) => {
                            println!("Received a request: {}", String::from_utf8_lossy(&buffer));
                            match Request::try_from(&buffer[..]) {
                                Ok(request) => {
                                    dbg!(request);
                                    let response =
                                        Response::new(StatusCode::Ok, Some("<h1>It works!</h1>".to_string()));
                                    write!(stream, "{}",response).unwrap();
                                }
                                Err(e) => { println!("Fail to parse request :{}", e); }
                            }
                        }
                        Err(e) => println!("Failed to read from connection: {}", e),
                    }
                }
                Err(e) => println!("Failed to establish a connection: {}", e),
            }
        }
    }
}
