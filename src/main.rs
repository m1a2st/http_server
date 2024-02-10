struct Server {
    addr: String,
}

impl Server {
    fn new(addr: String) -> Self {
        Server {
            addr,
        }
    }

    fn run(self) {
        println!("Server is running on {}", self.addr);
    }
}

fn main() {
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH
}