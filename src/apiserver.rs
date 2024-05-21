pub mod menusettings;

use std::net::TcpListener;
use std::sync::mpsc::Sender;
use maths_rs::num::Float;
use std::collections::HashMap;
use std::net::TcpStream;
use std::io::prelude::*;
use crate::apiserver::menusettings::MenuSettings;

pub fn start_server(tx : Sender<MenuSettings>) -> std::io::Result<()> {
    let listener =TcpListener::bind("127.0.0.1:8089").unwrap();

    let router = Router::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        Request::handle_request(&tx, stream, &router);
    }
    Ok(())
}

pub struct Router{
    routes: HashMap<String, (&'static str, fn(tx : &Sender<MenuSettings>, request : String) -> String)>
}

impl Router {
    pub fn new() -> Self {
        let mut routes: HashMap<String, (&'static str, fn(tx : &Sender<MenuSettings>, request : String) -> String)> = HashMap::new();
        routes.insert("/api/menu".to_string(), ("HTTP/1.1 200 OK", MenuSettings::update_settings) );
        routes.insert("/".to_string(), ("HTTP/1.1 404 NOT FOUND", |tx : &Sender<MenuSettings>, request: String| {"{ \"message\" : \"Resource not found\" }".to_string()}) );
        Router { routes }
    }

    fn route(&self, request: &str) -> &(&str, fn(tx : &Sender<MenuSettings>, request : String) -> String) {
        match self.routes.get(request) {
            Some(value) => value,
            None => {
                self.routes
                    .get("/")
                    .expect("Default route not found")
            }
        }
    }
}

pub struct Request {
    buffer: [u8; 1024],
    method: String,
    path: String
}

impl Request {
    pub fn handle_request(tx : &Sender<MenuSettings>, mut stream: TcpStream, router: &Router) {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer).unwrap();

        let request_str = String::from_utf8_lossy(&buffer).to_string();
        let parts: Vec<&str> = request_str.split_whitespace().collect();
        let ( method, path) = if parts.len() > 1 { (parts[0], parts[1]) } else { ("POST", "/") };

        let response = Request {
            buffer,
            method : method.to_string(),
            path : path.to_string(),
        }
            .router(tx, router);

        response.send_response(&stream);
    }

    fn router(&self, tx : &Sender<MenuSettings>, router: &Router) -> Response {
        let (status, json_data) = router.route(&self.path);

        //Executing our function
        let request_str = String::from_utf8_lossy(&self.buffer).to_string();
        let response_content = json_data(&tx, request_str);

        Response{
            status : status.to_string(),
            contents_size: response_content.len(),
            contents : response_content
        }
    }
}

pub struct Response {
    status: String,
    contents: String,
    contents_size: usize,
}

impl Response {

    pub fn send_response(&self, mut stream: &TcpStream){
        let response = self.deserialize_into_string();
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn deserialize_into_string(&self) -> String {
        format!("{}\r\nContent-Type: {}\r\nContent-Length: {}\r\n\r\n{}", self.status,"application/json" , self.contents_size, self.contents)
    }
}


