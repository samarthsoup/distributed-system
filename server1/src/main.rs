use std::net::{TcpStream, TcpListener};
use std::io::{BufRead, BufReader, Write};
use std::thread;

fn main() {
    let handle1 = thread::spawn(move || {
        if let Ok(mut stream) = TcpStream::connect("localhost:3000") {
            let arnold = "ALIVE\n";
            let _ = stream.write_all(arnold.as_bytes());
        } else {}
    });

    let handle2 = thread::spawn(move || {
        let client_comm_addr = format!("127.0.0.1:8080");
        let listener = TcpListener::bind(client_comm_addr).unwrap();

        for client_stream in listener.incoming() {
            let client_stream = client_stream.unwrap();

            handle_connection_client(client_stream);
        }
    });

    handle1.join().expect("The first thread has panicked");

    handle2.join().expect("The second thread has panicked");
}

fn handle_connection_client(mut stream: TcpStream) {
    let hello_message = "Hello, client!\n";
    if let Err(e) = stream.write_all(hello_message.as_bytes()) {
        eprintln!("failed to write to connection: {}", e);
        return;
    }

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
