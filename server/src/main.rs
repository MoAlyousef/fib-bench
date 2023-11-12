use ascii::AsciiString;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn get_content_type(path: &Path) -> &'static str {
    let extension = match path.extension() {
        None => return "text/plain",
        Some(e) => e,
    };

    match extension.to_str().unwrap() {
        "gif" => "image/gif",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "png" => "image/png",
        "pdf" => "application/pdf",
        "htm" => "text/html; charset=utf8",
        "html" => "text/html; charset=utf8",
        "txt" => "text/plain; charset=utf8",
        "js" => "application/javascript",
        "wasm" => "application/wasm",
        _ => "text/plain; charset=utf8",
    }
}

pub struct Server {
    port: u16,
    root: PathBuf,
    static_serve: bool,
    routes: HashMap<
        (tiny_http::Method, String),
        fn(rq: &mut tiny_http::Request) -> tiny_http::ResponseBox,
    >,
}

impl Server {
    pub fn new(port: u16, root: &Path) -> Self {
        Self {
            port,
            root: PathBuf::from(root),
            static_serve: false,
            routes: HashMap::new(),
        }
    }
    pub fn route(
        &mut self,
        verb: tiny_http::Method,
        url: &str,
        f: fn(rq: &mut tiny_http::Request) -> tiny_http::ResponseBox,
    ) {
        self.routes.insert((verb, url.to_string()), f);
    }
    pub fn static_serve(&mut self, flag: bool) {
        self.static_serve = flag;
    }
    pub fn serve(&mut self) {
        let server = tiny_http::Server::http(&format!("0.0.0.0:{}", self.port)).unwrap();
        while let Ok(mut rq) = server.recv() {
            let method = rq.method().clone();
            let url = rq.url().to_string();
            if let Some(f) = self.routes.get(&(method.clone(), url)) {
                let resp = f(&mut rq);
                let _ = rq.respond(resp);
            } else if self.static_serve && method == tiny_http::Method::Get {
                let mut url = rq.url().to_string().strip_prefix('/').unwrap().to_string();
                if url.is_empty() {
                    url = "index.html".to_string();
                }
                let path = Path::new(&url);
                let npath = self.root.join(path);
                let file = fs::File::open(&npath);
                if let Ok(file) = file {
                    let response = tiny_http::Response::from_file(file);
                    let response = response.with_header(tiny_http::Header {
                        field: "Content-Type".parse().unwrap(),
                        value: AsciiString::from_ascii(get_content_type(path)).unwrap(),
                    });
                    let _ = rq.respond(response);
                } else {
                    let rep = tiny_http::Response::new_empty(tiny_http::StatusCode(404));
                    let _ = rq.respond(rep);
                }
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Listening on 0.0.0.0:8000");
    let mut server = Server::new(8000, &PathBuf::from(&args[1]));
    server.route(tiny_http::Method::Post, "/", |rq| {
        let mut content = String::new();
        rq.as_reader().read_to_string(&mut content).unwrap();
        let n = fibonacci(content.parse().unwrap());
        tiny_http::Response::from_string(n.to_string()).boxed()
    });
    server.static_serve(true);
    server.serve();
}
