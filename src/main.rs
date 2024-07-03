// Uncomment this block to pass the first stage
use std::fs::*;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::*;

use itertools::Itertools;

struct Request {
    request_line: String,
    host_name: String,
    headers: Vec<String>,
}
impl Request {
    fn new(stream: &mut TcpStream) -> Self {
        let buf_reader = BufReader::new(stream);
        let http_request: Vec<_> = buf_reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        Self {
            request_line: http_request[0].clone(),
            host_name: http_request[1].clone(),
            headers: http_request[2..].to_vec(),
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let http_request = Request::new(&mut stream);

                let foo = match parse_request(&http_request) {
                    Ok(value) => {}
                    Err(e) => {}
                };

                let (_method, request_target) = parse_request_line(http_request.request_line);

                handle_request(&mut stream, request_target);

                crlf(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_request(stream: &mut TcpStream, request_target: String) {
    let mut split_target = request_target
        .split('/')
        .filter(|string| !string.is_empty());

    if let Some(cmd) = split_target.clone().next() {
        match cmd.trim().to_lowercase().as_str() {
            "echo" => {
                let content = split_target.clone().skip(1).next().unwrap();
                let length = &content.len();
                let content_type = "text/plain";

                let response = format!(
                    "Content-Type: {content_type}\r\nContent-Length: {length}\r\n\r\n{content}"
                );
                connection_ok(stream);
                stream.write(response.as_bytes()).unwrap();
                return;
            }
            "user-agent" => {
                // TODO - capture Usar-Agent header value
            }
            _ => {}
        }
    }

    println!("Looking for page: {}", request_target);

    let joined_target = split_target.join(MAIN_SEPARATOR_STR);
    let target_path = if joined_target.is_empty() {
        Path::new(MAIN_SEPARATOR_STR)
    } else {
        Path::new(&joined_target)
    };

    println!("Target Path: {:?}", target_path);

    if let Ok(metadata) = target_path.metadata() {
        connection_ok(stream);
        if metadata.is_dir() {
            println!("This is a directory. Looking for index.html");
            match read_to_string("index.html") {
                Ok(_content) => {}
                Err(_) => {}
            }
        }
        if metadata.is_file() {
            println!("This is a file. Attempt to retrieve content.");
            match read_to_string(target_path) {
                Ok(content) => {
                    let length = &content.len();
                    let response = format!("Content-Length: {length}\r\n\r\n{content}");
                    stream.write(response.as_bytes()).unwrap();
                }
                Err(_) => {
                    println!("Error reading page contents!");
                    not_found(stream);
                    return;
                }
            }
        }
    } else {
        not_found(stream);
    }
}

fn parse_request(request: &Request) -> Result<(String, String, Vec<String>), String> {
    let headers = request.headers.clone();

    if let Some((method, target, _)) = request.request_line.split_whitespace().collect_tuple() {
        return Ok((method.to_string(), target.to_string(), headers));
    }

    return Err("Malformed request!".to_string());
}

fn parse_request_line(request_line: String) -> (String, String) {
    if let Some((func, args, _)) = request_line.split_whitespace().collect_tuple() {
        return (func.to_string(), args.to_string());
    }
    return ("".to_string(), "".to_string());
}

fn connection_ok(stream: &mut TcpStream) -> usize {
    stream.write(b"HTTP/1.1 200 OK\r\n").unwrap()
}

fn not_found(stream: &mut TcpStream) -> usize {
    stream.write(b"HTTP/1.1 404 Not Found\r\n").unwrap()
}

fn crlf(stream: &mut TcpStream) -> usize {
    stream.write(b"\r\n").unwrap()
}
