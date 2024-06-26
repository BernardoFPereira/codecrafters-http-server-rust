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

    // match listener.accept() {
    //     Ok((mut stream, addr)) => {
    //         connection_ok(&mut stream);
    //         println!("{addr:?}");
    //         // stream.write_all(b"{addr:?}").unwrap();
    //     }
    //     Err(e) => {}
    // }

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

                let (_func, mut args) = parse_request(&http_request);
                let args_len = args.len();
                args.retain(|c| if args_len > 1 { c != '/' } else { true });

                println!("Looking for: {}", args);

                if args == "/".to_string() {
                    connection_ok(&mut stream);
                    crlf(&mut stream);
                    continue;
                }

                if let Ok(_) = read_to_string(args) {
                    connection_ok(&mut stream);
                } else {
                    not_found(&mut stream);
                }
                // if args != "/" {
                //     not_found(&mut stream);
                //     crlf(&mut stream);
                //     continue;
                // }

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
