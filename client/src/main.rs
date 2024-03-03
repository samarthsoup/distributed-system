use std::net::TcpStream;
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
    loop{
        let input = match process_input() {
            Ok(input) => input,
            Err(e) => {
                eprintln!("error: {}", e);
                continue;
            }
        };

        let mut stream = TcpStream::connect("localhost:3000").unwrap();
        stream.write_all(input.as_bytes()).unwrap();
        stream.flush().unwrap();

        let mut buf_reader = BufReader::new(&stream);
        let mut response = String::new();

        buf_reader.read_line(&mut response).unwrap();

        println!("server: {response}");
    }
}
