use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let parameters: Vec<String> = env::args().collect();
    let pre_option_elected = &parameters[1];
    let number_of_threads = parameters[3].parse::<usize>().unwrap();
    println!("{:?}",parameters);
    let mut ip_adress = "127.0.0.1:".to_owned();
    ip_adress.push_str(&parameters[7]);
    println!("{:?}",ip_adress);

    let listener = TcpListener::bind(ip_adress).unwrap();
    let pool = ThreadPool::new(number_of_threads);

    for stream in listener.incoming().take(number_of_threads) { 
        let stream = stream.unwrap();

        pool.execute(|| {
            let parameters: Vec<String> = env::args().collect();
            let files_path = &parameters[5];
            handle_connection(stream, files_path.to_string());
        });
    }

    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream, files_path: String) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "html files/helloWorld.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "html files/helloWorld.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "html files/error404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}