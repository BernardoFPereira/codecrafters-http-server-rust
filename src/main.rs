// Uncomment this block to pass the first stage
use std::fs::*;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::path::*;

use itertools::Itertools;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");

                let buf_reader = BufReader::new(&mut stream);
                let http_request: Vec<_> = buf_reader
                    .lines()
                    .map(|result| result.unwrap())
                    .take_while(|line| !line.is_empty())
                    .collect();

                let (_method, request_target) = parse_request(&http_request);
                let mut split_request = request_target
                    .split('/')
                    .filter(|string| !string.is_empty());

                println!("Looking for page: {}", request_target);

                let joined_target = split_request.join(MAIN_SEPARATOR_STR);

                let target_path = if joined_target.is_empty() {
                    Path::new(MAIN_SEPARATOR_STR)
                } else {
                    Path::new(&joined_target)
                };

                println!("Target Path: {:?}", target_path);

                if let Ok(metadata) = target_path.metadata() {
                    connection_ok(&mut stream);
                    if metadata.is_dir() {
                        println!("This is a directory. Looking for index.html");
                    }
                    if metadata.is_file() {
                        println!("This is a file. Attempt to retrieve content.");
                        match read_to_string(target_path) {
                            Ok(content) => {
                                let length = &content.len();
                                let response =
                                    format!("Content-Length: {}\r\n\r\n{}", length, content);
                                stream.write(response.as_bytes()).unwrap();
                            }
                            Err(_) => {
                                println!("Error reading page contents!");
                                not_found(&mut stream);
                                continue;
                            }
                        }
                    }
                } else {
                    not_found(&mut stream);
                }

                crlf(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
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

fn parse_request(request: &Vec<String>) -> (String, String) {
    if let Some((func, args, _)) = request
        .iter()
        .next()
        .unwrap_or(&String::from("."))
        .split_whitespace()
        .collect_tuple()
    {
        // println!("Command: {}\nArgs: {}", func, args);
        return (func.to_string(), args.to_string());
    }
    return ("".to_string(), "".to_string());
}
