use std::net::{TcpStream, TcpListener, Shutdown};
use std::io::{BufRead, BufReader, Write};
use std::sync::{mpsc, Arc, atomic::{AtomicBool, Ordering}};
use std::{thread};

fn main() {
    let should_disconnect = Arc::new(AtomicBool::new(false));

    let should_disconnect_for_client_thread = Arc::clone(&should_disconnect);
    let handle1 = thread::spawn(move || {
        let client_addr = "127.0.0.1:8081";
        let listener = TcpListener::bind(client_addr).unwrap();

        for client_stream in listener.incoming() {
            let stream = client_stream.unwrap();
            let should_disconnect = Arc::clone(&should_disconnect_for_client_thread);
            thread::spawn(move || {
                handle_connection(stream, should_disconnect);
            });
        }
    });

    let handle2 = thread::spawn(move || {
        let server_comm_addr = "127.0.0.1:3000";
        let listener = TcpListener::bind(server_comm_addr).unwrap();

        for server_stream in listener.incoming() {
            let server_stream = server_stream.unwrap();
            handle_connection_server(server_stream, Arc::clone(&should_disconnect));
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn handle_connection(mut stream: TcpStream, should_disconnect: Arc<AtomicBool>) {
    loop {
        if should_disconnect.load(Ordering::SeqCst) {
            let _ = stream.shutdown(Shutdown::Both);
            break;
        }

        let mut line = String::new();
        let mut buf_reader = BufReader::new(&stream);
        match buf_reader.read_line(&mut line) {
            Ok(0) => {
                break;
            }
            Ok(_) => {
                let line = line.trim_end(); 
                println!("Client: {}", line);
            }
            Err(e) => {
                eprintln!("Failed to read from connection: {}", e);
                break;
            }
        }

        stream.write_all(line.as_bytes()).unwrap();
    }
}

fn handle_connection_server(mut stream: TcpStream, should_disconnect: Arc<AtomicBool>) {
    let mut line = String::new();
    let mut buf_reader = BufReader::new(&stream);
    match buf_reader.read_line(&mut line) {
        Ok(_) => {
            let line = line.trim_end();
            println!("Server: {}", line);
            if line == "ALIVE" {
                should_disconnect.store(true, Ordering::SeqCst);
            }
        }
        Err(e) => {
            eprintln!("Failed to read from server connection: {}", e);
        }
    }
}
