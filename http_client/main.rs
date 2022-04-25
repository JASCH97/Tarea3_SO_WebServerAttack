use std::{convert::Infallible, net::SocketAddr};
use hyper::{Body, Request, Response, Server};
use hyper::service::{make_service_fn, service_fn};
use hyper::{Client, Method, Uri};
use std::fs;
use hyper::body;

async fn handle(_: Request<Body>) -> Result<Response<Body>, Infallible> {
    Ok(Response::new("Hello, World!".into()))
}

#[tokio::main]
/*
    Metodo que crea el cliente y lo conecta al server con la direccion ip y el puerto que corresponda,
    No tiene parametros e indica un error en caso de que suceda.
*/
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 7878));

    let make_svc = make_service_fn(|_conn| async {
        Ok::<_, Infallible>(service_fn(handle))
    });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

/*
    Funcion que procesa los metodos get del cliente 
*/
async fn get(url:&str, filename: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let request = Request::builder()
        .method(Method::GET)
        .uri(url)
        .header("accept", "application/json")
        .body(Body::from(filename.to_string())).unwrap();
    let client = Client::new();
    let resp = client.request(request).await.unwrap();
    println!("Response GET: {}", resp.status());
    let bytes = body::to_bytes(resp.into_body()).await.unwrap();
    //println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );
    Ok(())
}

/*
    Funcion que procesa los metodos post del cliente 
*/
async fn post(url: &str, message: &str) ->  Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let data = fs::read_to_string(message).expect("Unable to read file");

    let request = Request::builder()
        .method(Method::POST)
        .uri(url)
        .header("accept", "application/json")
        .header("Content-type", "application/json; charset=UTF-8")
        .body(Body::from(data)).unwrap();
    let client = Client::new();
    let resp = client.request(request).await.unwrap();
    let bytes = body::to_bytes(resp.into_body()).await.unwrap();

    println!("GOT BYTES: {}", std::str::from_utf8(&bytes).unwrap() );
    Ok(())

}