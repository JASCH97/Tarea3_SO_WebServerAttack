use std::io::{stdout, Write};
use curl::easy::Easy;
use std::thread;
use std::env;
use std::process::Command;
use std::time::Duration;
use reqwest;

#[path = "./prethread-webserver.rs"] mod prethreadwebserver;
#[path = "./preforked-webserver.rs"] mod preforkedwebserver;

//Metodo main, se encarga de recibir los parametros y llamar los metodos requeridos por el usuario
pub fn main(){
  env::set_var("RUST_BACKTRACE", "1");
  let args: Vec<String> = env::args().collect();
  // Write the contents of rust-lang.org to stdout
  let mut easy = Easy::new();
  let mut i = 1;
  if args[i] == "-h"{
     println!("Bienvenido al HTTPClient, con este programa se conectara al servidor usando la libreria curl \n\n");
     i = i + 1;
  }
  println!("{}",args[i]);
  if args[i] == "get"{
     let mut puerto = "3000";
     let mut archivo = "holamundo.html";
     //genera un request tipo get al servidor en el puerto dado
     if args.len() > i+2{
       let puerto:&str = &args[i+1];
       let archivo:&str = &args[i+2];
     }
     let mut listen =String::new();
     listen.push_str("127.0.0.1:");
     listen.push_str(puerto);
     listen.push_str("/");
     listen.push_str(archivo);
     easy.url(&listen).unwrap();
     easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
     }).unwrap();
     easy.perform().unwrap();
     
  } else if args[i] == "prethread-webserver"{
    //inicia el servidor por medio de un hilo y en el otro hilo realiza un request al servidor
     thread::spawn(|| {
        prethreadwebserver::main();
        thread::sleep(Duration::from_millis(1));
     });
     thread::sleep(Duration::from_millis(10));
     easy.url("http://localhost:3000/").unwrap();
     easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
     }).unwrap();
     easy.perform().unwrap();
     thread::sleep(Duration::from_millis(1));
     
  } else {
    //inicia el servidor por medio de un hilo y en el otro hilo realiza un request al servidor
     thread::spawn(|| {
        preforkedwebserver::main();
        thread::sleep(Duration::from_millis(1));
     });
     thread::sleep(Duration::from_millis(10));
     easy.url("http://localhost:3000/").unwrap();
     easy.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
     }).unwrap();
     easy.perform().unwrap();
     thread::sleep(Duration::from_millis(1));
  }
}



