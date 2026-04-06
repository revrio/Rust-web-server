use simple_server::ThreadPool;
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};
use std::sync::{Arc, Mutex};

fn main() {
    let listener = TcpListener::bind("0.0.0.0:8000").unwrap();
    let pool = ThreadPool::new(4);

    let counter = Arc::new(Mutex::new(0)); 

    println!("Server is listening on port 8000!");
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let counter_clone = Arc::clone(&counter);
        pool.execute(move || {
            handle_connection(stream, counter_clone);
        });
    }
}

fn handle_connection(mut stream: TcpStream, counter: Arc<Mutex<usize>>) {
    let buf_reader = BufReader::new(&stream);
    
    let request_line = match buf_reader.lines().next() {
        Some(Ok(line)) => line,
        _ => return,
    };

    let ip = stream.peer_addr().unwrap().ip();
    println!("[LOG] Connection from IP: {} | Request: {}", ip, request_line);

    let parts: Vec<&str> = request_line.split_whitespace().collect();
    let path = if parts.len() > 1 { parts[1] } else { "/" };

    let filename = if path == "/" { 
        "public/index.html".to_string() 
    } else { 
        format!("public{}", path) 
    };

    let (status_line, mut contents, content_type) = match fs::read(&filename) {
        Ok(data) => {
            let mime = if filename.ends_with(".css") { "text/css" }
                       else if filename.ends_with(".js") { "application/javascript" }
                       else if filename.ends_with(".png") { "image/png" }
                       else { "text/html" };
            ("HTTP/1.1 200 OK", data, mime)
        },
        Err(_) => {
            let data = fs::read("public/404.html").unwrap_or_else(|_| b"404 Not Found".to_vec());
            ("HTTP/1.1 404 NOT FOUND", data, "text/html")
        }
    };

    if filename == "public/index.html" {
        let mut num = counter.lock().unwrap();
        *num += 1;
        
        let mut html_string = String::from_utf8_lossy(&contents).to_string();
        html_string = html_string.replace("</h1>", &format!("</h1>\n<p><strong>You are visitor #{}!</strong></p>", *num));
        contents = html_string.into_bytes();
        println!("Visitor count updated: {}", *num);
    }
    let length = contents.len();
    let response_header = format!(
        "{}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n", 
        status_line, length, content_type
    );

    stream.write_all(response_header.as_bytes()).unwrap();
    stream.write_all(&contents).unwrap();
}