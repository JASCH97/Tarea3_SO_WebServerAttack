//bibliotecas utilizadas
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::env;
use std::fs;
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    //Se define el backstrace para evitar errores de ejecucion
    env::set_var("RUST_BACKTRACE", "1");          
    
    //se toman los parametros ingresados en consola y se guardan en variables
    let parameters: Vec<String> = env::args().collect();  
    //let pre_option_elected = &parameters[1];                                //con esto sabriamos si se trata de prethread o preforked
    let number_of_threads = parameters[3].parse::<usize>().unwrap();
    println!("{:?}",parameters);
    let mut ip_adress = "127.0.0.1:".to_owned();
    ip_adress.push_str(&parameters[7]);

    //con TcpListener podemos 'escuchar' las conexiones que se dan en la ip:puerto seleccionados
    let listener = TcpListener::bind(ip_adress).unwrap();
    //con la libreria ThreadPool creada se manejan los hilos del servidor
    let pool = ThreadPool::new(number_of_threads);

    //ciclo que maneja los hilos y las peticiones hasta que se llegue al limite
    for stream in listener.incoming().take(number_of_threads) { 
        let stream = stream.unwrap();

        pool.execute(|| {
            let parameters: Vec<String> = env::args().collect();
            let files_path = &parameters[5];
            handle_requests(stream, files_path.to_string());
        });
    }

    println!("Shutting down.");
}

//Esta funcion maneja las peticiones y utiliza el directorio de archivos sobre el que se desea trabajar
fn handle_requests(mut stream: TcpStream, files_path: String) {
    
    //se utiliza un buffer para contener los datos que se leen y luego imprimirlo en consola
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    
    //Se definen los distintos tipos de solicitudes para el servidor http
    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";
    let post = b"GET /post HTTP/1.1\r\n";
    
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", files_path + "/helloWorld.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", files_path + "/helloWorld.html")
    } else if buffer.starts_with(post) {
        ("HTTP/1.1 200 OK", files_path + "/helloWorld.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", files_path + "/error404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );
    //con write se toman los bytes de informacion y se envian a la conexion
    stream.write(response.as_bytes()).unwrap();
    //flush impide que se continue la ejecucion si no se terminan de escribir los bytes
    stream.flush().unwrap();
}