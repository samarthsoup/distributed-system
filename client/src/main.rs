use std::net::{TcpStream, SocketAddr};
use std::io::{self, prelude::*};
use std::io::BufReader;


fn process_input() -> Result<String, io::Error> {
    print!("? ");
    io::stdout().flush()?;

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input.to_string()), 
        Err(e) => Err(e),
    }
}


fn main() {
    let addrs = [
        SocketAddr::from(([127, 0, 0, 1], 8080)),
        SocketAddr::from(([127, 0, 0, 1], 8081)),
    ];
    let mut stream = TcpStream::connect(&addrs[..]).unwrap();
    loop{
        let input = match process_input() {
            Ok(input) => input,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };
        
        if let Err(e) = stream.write_all(input.as_bytes()).and_then(|_| stream.flush()) {
            eprintln!("server down: {e}");
            println!("attempting to connect to another server");
            stream = match TcpStream::connect(&addrs[..]) {
                Ok(stream) => {
                    println!("connected at {}", stream.peer_addr().unwrap());
                    stream
                }
                Err(e) => {
                    eprintln!("couldn't connect to both servers: {e}");
                    return;
                }
            };
            stream.write_all(input.as_bytes()).unwrap();
            stream.flush().unwrap();
        }

        let mut buf_reader = BufReader::new(&stream);
        let mut response = String::new();

        if let Err(_) = buf_reader.read_line(&mut response) {
            stream = match TcpStream::connect(&addrs[..]) {
                Ok(stream) => {
                    println!("connected at {}", stream.peer_addr().unwrap());
                    stream
                }
                Err(e) => {
                    eprintln!("couldn't connect to both servers: {e}");
                    return;
                }
            };
            let mut buf_reader = BufReader::new(&stream);
            match buf_reader.read_line(&mut response) {
                Ok(_) => println!("server: {}", response),
                Err(e) if e.kind() == io::ErrorKind::WouldBlock || e.kind() == io::ErrorKind::TimedOut => {
                    println!("server: {}", response);
                },
                Err(e) => eprintln!("Error reading from server: {}", e),
            }
        };

       
    }
}
