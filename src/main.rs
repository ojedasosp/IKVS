use std::{
    io::{self},
    net::{TcpListener, TcpStream},
    thread,
};

mod resp;
use resp::{Reader, Value, Writer};

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

fn handle_client(stream: TcpStream) {
    let rstream = match stream.try_clone() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("couldn't clone stream: {e}");
            return;
        }
    };

    let mut reader = Reader::new(rstream);
    let mut writer = Writer::new(stream);

    loop {
        let value = match reader.read() {
            Ok(v) => v,
            Err(e) if e.kind() == io::ErrorKind::UnexpectedEof => break,
            Err(e) => {
                eprintln!("read error: {e}");
                break;
            }
        };

        println!("{value:#?}");

        if let Err(e) = writer.write(Value {
            typ: "string".to_string(),
            str: "OK".to_string(),
            ..Default::default()
        }) {
            eprint!("error writing to client: {e}");
            break;
        }
    }
}
