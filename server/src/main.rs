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
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    println!("msg: {request_line}"); 

    stream.write_all(request_line.as_bytes()).unwrap();
}
