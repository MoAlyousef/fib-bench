use livid_server::{Server, Method, Response};

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("Listening on 127.0.0.1:{}", args[1]);
    let mut server = Server::new(args[1].parse().unwrap());
    server.route(Method::Post, "/", |rq| {
        let mut content = String::new();
        rq.as_reader().read_to_string(&mut content).unwrap();
        let n = fibonacci(content.parse().unwrap());
        Response::from_string(n.to_string()).boxed()
    });
    server.serve_dir(&args[2]);
    server.serve();
}
