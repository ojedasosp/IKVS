use std::{
    io::{self, Write},
    net::{TcpListener, TcpStream},
    thread,
};

mod resp;
use resp::Resp;

fn main() -> io::Result<()> {
    let listener = match TcpListener::bind("127.0.0.1:6379") {
        Ok(l) => l,
        Err(e) => {
            eprintln!("couldn't get client: {e}");
            std::process::exit(1);
        }
    };
    println!("Listening on port {}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("couldn't accept client: {e}");
                std::process::exit(1);
            }
        }
    }

    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let reader = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("couldn't clone stream: {e}");
            return;
        }
    };

    let mut resp = Resp::new(reader);

    loop {
        let value = match resp.read() {
            Ok(v) => v,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                eprintln!("read error: {e}");
                break;
            }
        };

        print!("{value:#?}");

        if let Err(e) = stream.write(b"+OK\r\n") {
            eprint!("error writing to client: {e}");
            break;
        }
    }
}
