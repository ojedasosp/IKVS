use std::{
    default,
    io::{self, BufReader, Read},
    net::TcpStream,
};

const SIMPLE: u8 = b'+';
const ERROR: u8 = b'-';
const INT: u8 = b':';
const BULK: u8 = b'$';
const ARRAY: u8 = b'*';

#[derive(Debug, Default)]
pub struct Value {
    pub typ: String,
    str: String,
    num: u32,
    bulk: String,
    array: Vec<Value>,
}

impl Value {
    fn null() -> Value {
        Value {
            typ: String::from("null"),
            ..Default::default()
        }
    }
}

const MAX_BULK_LEN: i64 = 512 * 1024 * 1024;

// Wraps any error (or &str) into an io::Error with InvalidData
fn invalid<E: Into<Box<dyn std::error::Error + Send + Sync>>>(e: E) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, e)
}

pub struct Resp {
    reader: BufReader<TcpStream>,
}

impl Resp {
    pub fn new(stream: TcpStream) -> Resp {
        return Resp {
            reader: BufReader::new(stream),
        };
    }

    fn read_line(&mut self) -> io::Result<Vec<u8>> {
        let mut line = Vec::new();
        for b in self.reader.by_ref().bytes() {
            line.push(b?);
            if line.ends_with(b"\r\n") {
                line.truncate(line.len() - 2);
                return Ok(line);
            }
        }

        Err(io::Error::new(
            io::ErrorKind::UnexpectedEof,
            "connection closed",
        ))
    }

    fn read_integer(&mut self) -> io::Result<i64> {
        let line = self.read_line()?;
        let text = std::str::from_utf8(&line).map_err(invalid)?;
        text.parse::<i64>().map_err(invalid)
    }

    pub fn read(&mut self) -> io::Result<Value> {
        let mut typ = [0u8; 1];
        self.reader.read_exact(&mut typ)?;
        return match typ[0] {
            ARRAY => self.read_array(),
            BULK => self.read_bulk(),
            _ => Err(io::Error::new(io::ErrorKind::InvalidData, "Unknow type")),
        };
    }

    fn read_array(&mut self) -> io::Result<Value> {
        let length = self.read_integer()?;
        let mut array = Vec::with_capacity(length.min(1024) as usize);
        for _ in 0..length {
            array.push(self.read()?);
        }

        Ok(Value {
            typ: String::from("array"),
            array,
            ..Default::default()
        })
    }

    fn read_bulk(&mut self) -> io::Result<Value> {
        let length = self.read_integer()?;
        if length < 0 {
            return Ok(Value::null());
        }
        if length > MAX_BULK_LEN {
            return Err(invalid("bulk size too large"));
        }
        let mut buf = vec![0u8; length as usize];
        self.reader.read_exact(&mut buf)?;

        let bulk = String::from_utf8(buf).map_err(invalid)?;

        self.read_line()?;

        Ok(Value {
            typ: String::from("bulk"),
            bulk,
            ..Default::default()
        })
    }
}
