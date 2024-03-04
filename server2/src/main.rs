use std::net::{TcpStream, TcpListener, Shutdown};
use std::io::{BufRead, BufReader, Write};
use std::sync::mpsc;
use std::{thread};

fn main() {
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let server_comm_addr = "127.0.0.1:3000";
        let listener = TcpListener::bind(server_comm_addr).unwrap();

        for server_stream in listener.incoming() {
            let server_stream = server_stream.unwrap();
            handle_connection_server(server_stream, tx.clone());
        }
    });
    
    let client_addr = "127.0.0.1:8081";
    let listener = TcpListener::bind(client_addr).unwrap();

    for client_stream in listener.incoming() {
        let stream = client_stream.unwrap();
        
        handle_connection(stream, &rx);
    }

    thread::park();
}

fn handle_connection(mut stream: TcpStream, rx: &mpsc::Receiver<&str>) {
    loop {
        match rx.try_recv() {
            Ok("ALIVE") => stream.shutdown(Shutdown::Both).unwrap(),
            _ => {}
        }

        let mut line = String::new();
        let mut buf_reader = BufReader::new(&stream);
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

        stream.write_all(line.as_bytes()).unwrap();
    }
}

fn handle_connection_server(mut stream: TcpStream, tx: mpsc::Sender<&str>) {
    let mut line = String::new();
    let mut buf_reader = BufReader::new(&stream);
    match buf_reader.read_line(&mut line) {
        Ok(_) => {
            let line = line.trim_end();
            println!("server1: {}", line);
            if line == "ALIVE" {
                tx.send("ALIVE").unwrap();
            }
        }
        Err(e) => {
            eprintln!("failed to read from server connection: {}", e);
        }
    }
}