use std::process;
use std::env;
use std::net::{TcpStream, TcpListener};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let mut args_iter = env::args().into_iter();
    args_iter.next();

    let port = match args_iter.next(){
        Some(port) => port,
        None => {
            eprintln!("provide a port");
            process::exit(1);
        }
    };

    let addr = format!("127.0.0.1:{}", port);
    let listener = TcpListener::bind(addr).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    loop {
        let mut line = String::new();
        {
            let mut buf_reader = BufReader::new(&mut stream);
            match buf_reader.read_line(&mut line) {
                Ok(0) => {
                    break;
                }
                Ok(_) => {
                    let line = line.trim_end(); 
                    println!("client: {}", line);
                }
                Err(e) => {
                    eprintln!("failed to read from connection: {}", e);
                    break;
                }
            }
        }

        stream.write_all(line.as_bytes()).unwrap();
    }
}
