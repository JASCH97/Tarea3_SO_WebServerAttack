use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut cantPro:i32 = 1;
    if args.len() > 1 {
        let mut cantPro: i32 = args[2].parse().unwrap();
    }
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!\n\n");
        if cantPro > 0{
            cantPro = cantPro -1;
        handle_connection(stream);
        }
    }
}

pub fn path_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();

    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    
    let get = b"GET";
    let head = b"HEAD";
    let post = b"POST";
    let put = b"PUT";
    let delete = b"DELETE";
    let requestT = String::from_utf8_lossy(&buffer[..]);
    if buffer.starts_with(get) {
        let message:Vec<&str>= requestT.split(" ").collect();
        let mut getFile:String = message[1].to_string();
        getFile.remove(0);
        if path_exists(&getFile){
            let contents = fs::read_to_string(getFile).unwrap();

            let response = format!(
                 "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                 contents.len(),
                 contents
            );
            println!("{}",response);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
           let status_line = "HTTP/1.1 404 NOT FOUND";
           let contents = fs::read_to_string("404.html").unwrap();

           let response = format!(
               "{}\r\nContent-Length: {}\r\n\r\n{}",
               status_line,
               contents.len(),
               contents
           );
           stream.write(response.as_bytes()).unwrap();
           stream.flush().unwrap();
        }
     } else if buffer.starts_with(head) {
        let message:Vec<&str>= requestT.split(" ").collect();
        let mut headFile:String = message[1].to_string();
        headFile.remove(0);
        if path_exists(&headFile){
            let contents = fs::read_to_string(headFile).unwrap();

            let response = format!(
                 "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n",
                 contents.len()
            );
            println!("{}",response);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
           let status_line = "HTTP/1.1 404 NOT FOUND";
           let contents = fs::read_to_string("404.html").unwrap();

           let response = format!(
               "{}\r\nContent-Length: {}\r\n\r\n",
               status_line,
               contents.len()
           );
           stream.write(response.as_bytes()).unwrap();
           stream.flush().unwrap();
        }
     
     } else if buffer.starts_with(post) {
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length:\r\n\r\n"
        );
        println!("{}",response);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        
     } else if buffer.starts_with(put) {
        let message:Vec<&str>= requestT.split(" ").collect();
        let mut putFile:String = message[1].to_string();
        putFile.remove(0);
        
        
        
        let contents = fs::read_to_string("allgood.html").unwrap();

        let response = format!(
           "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
           contents.len(),
           contents
        );
        println!("{}",response);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
     
     } else if buffer.starts_with(delete) {
        let message:Vec<&str>= requestT.split(" ").collect();
        let mut deleteFile:String = message[1].to_string();
        deleteFile.remove(0);
        if path_exists(&deleteFile){
            let contents = fs::remove_file(deleteFile);
            println!("File deleted successfully!");
            
            let contents = fs::read_to_string("allgood.html").unwrap();

            let response = format!(
                 "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                 contents.len(),
                 contents
            );
            println!("{}",response);
            stream.write(response.as_bytes()).unwrap();
            stream.flush().unwrap();
        } else {
           let status_line = "HTTP/1.1 404 NOT FOUND";
           let contents = fs::read_to_string("404.html").unwrap();

           let response = format!(
               "{}\r\nContent-Length: {}\r\n\r\n",
               status_line,
               contents.len()
           );
           stream.write(response.as_bytes()).unwrap();
           stream.flush().unwrap();
        }
     
     } else {
        let status_line = "HTTP/1.1 501 NOT IMPLEMENT";
        let contents = fs::read_to_string("501.html").unwrap();

        let response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}
