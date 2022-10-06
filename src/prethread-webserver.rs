use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use std::thread;
use std::env;
use std::time::Duration;

//Metodo main, se encarga de recibir los parametros y llamar los metodos requeridos por el usuario, se conecta al puerto deseado ademas se encarga de crear los hilos del servidor
pub fn main() {
    env::set_var("RUST_BACKTRACE", "FULL");
    let args: Vec<String> = env::args().collect();
    let mut hilos:i32 = 4;
    println!("{}", args.len());
    let mut puerto:i32 = 3000;
    //por medio de la cantidad de argumentos, obtenemos tanto la cantidad de hilos como el puerto, en caso que no ingrese esos argumentos se tomaran los valores por defecto
    if args.len() > 3 {
       if args[1] == "prethread-webserver"{
          hilos = args[2].parse().unwrap();
          puerto = args[3].parse().unwrap();
       }
    } else if args.len() > 2 {
       if args[1] == "prethread-webserver"{
          hilos = args[2].parse().unwrap();
       }
    }else if args.len() > 1 {
        if args[1] != "prethread-webserver"{
          hilos = args[1].parse().unwrap();
       }
    }else if args.len() > 2 {
       if args[1] != "prethread-webserver"{
          puerto = args[2].parse().unwrap();
       }
    }
    let mut listen =String::new();
    listen.push_str("127.0.0.1:");
    listen.push_str(&puerto.to_string());
    //Prepara la coneccion al puerto deseado
    let listener = TcpListener::bind(listen).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!\n\n");
        if hilos > 0{
            hilos = hilos -1;
            //Genera el hilo para el request obtenido
            thread::spawn(|| {
               handle_connection(stream);
               thread::sleep(Duration::from_millis(1));
            });
            hilos = hilos + 1;
        }
    }
}
//verifica si un archivo o path existe
pub fn path_exists(path: &str) -> bool {
    
    fs::metadata(path).is_ok()
}

//maneja los request depende de la peticion requerida genera y envia una respuesta al servidor
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
    //verifica si el mensaje del buffer es un get y crea la respuesta
    if buffer.starts_with(get) {
        let message:Vec<&str>= requestT.split(" ").collect();
        let mut getFile:String = message[1].to_string();
        getFile.remove(0);
        println!("{}",getFile);
        //si el archivo existe realiza la respuesta sino genera un error 404
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
           //si el archivo no existe envia la respuesta con el error 404
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
     //verifica si el mensaje del buffer es un head y crea la respuesta
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
        //verifica si el mensaje del buffer es un post y crea la respuesta
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length:\r\n\r\n"
        );
        println!("{}",response);
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
        
     } else if buffer.starts_with(put) {
        //verifica si el mensaje del buffer es un put y crea la respuesta
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
        //verifica si el mensaje del buffer es un delete, realiza el delete del archivo y obtiene una respuesta
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
