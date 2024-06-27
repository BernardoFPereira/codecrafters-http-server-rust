// Uncomment this block to pass the first stage
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

                let (_func, args) = parse_request(&http_request);

                let target_str = match args.strip_prefix("/") {
                    Some(dir) => {
                        format!(".{}{}", MAIN_SEPARATOR, dir)
                    }
                    None => MAIN_SEPARATOR_STR.to_string(),
                };

                let target_path = Path::new(&target_str);

                println!("Looking for page: {}", args);

                println!("target dir: {:?}", target_path);
                if let Ok(_) = target_path.metadata() {
                    connection_ok(&mut stream);
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
    if let Some((func, args, _)) = request[0].split_whitespace().collect_tuple() {
        println!("Command: {}\nArgs: {}", func, args);
        return (func.to_string(), args.to_string());
    }
    return ("".to_string(), "".to_string());
}
