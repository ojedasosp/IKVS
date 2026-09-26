mod handler;
mod resp;

use std::{
    io::{self},
    net::{TcpListener, TcpStream},
    thread,
};

use resp::{Reader, Value, Writer};

use crate::handler::HANDLERS;

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

        if value.typ != "array".to_string() {
            println!("Invalid request expected array");
            continue;
        }

        if value.array.len() == 0 {
            println!("Invalid request, expected array length > 0");
            continue;
        }

        let command = value.array[0].bulk.to_uppercase();
        let args = &value.array[1..];

        let reply = match HANDLERS.get(command.as_str()) {
            Some(handler) => handler(args),
            None => Value {
                typ: "error".to_string(),
                str: format!("ERR unknow command: {command}"),
                ..Default::default()
            },
        };

        println!("{value:#?}");

        if let Err(e) = writer.write(reply) {
            eprint!("error writing to client: {e}");
            break;
        }
    }
}
