// Uncomment this block to pass the first stage
use std::io::Write;
use std::net::{TcpListener, TcpStream};

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
                match listener.accept() {
                    Ok((socket, addr)) => {
                        println!("{socket:?}\n{addr:?}");
                        connection_ok(&mut stream);
                        crlf(&mut stream);
                    }
                    Err(e) => {
                        println!("error: {}", e);
                    }
                }
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

fn crlf(stream: &mut TcpStream) -> usize {
    stream.write(b"\r\n").unwrap()
}
