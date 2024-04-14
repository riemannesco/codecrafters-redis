use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::result::Result;
use std::thread;

enum Command {
    Ping,
    Echo,
}

#[derive(PartialEq, Debug)]
struct Request<'a> {
    request_type: RequestType,
    raw: &'a [u8],
    data: &'a [u8],
}

#[derive(PartialEq, Debug)]
enum RequestTypeError {
    UnknownRequestType,
    NoRequestType,
}

#[derive(PartialEq, Debug)]
enum RequestType {
    SString,
    //Integer,
    BString,
    //Array,
    Unknown,
}
impl<'a> Request<'a> {
    fn parse_command(command: &[u8]) -> Result<Request<'_>, Box<dyn std::error::Error>> {
        let prefix = command.first();
        let prefix = match prefix {
            Some(p) => p,
            None => return Err("empty request".into()),
        };

        let request_type = RequestType::build(prefix);
        if request_type.eq(&RequestType::Unknown) {
            return Err("unknown request type".into());
        }

        let args: Vec<&[u8]> = command
            .split(|&c| c == b'\n')
            .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
            .collect();

        let request: Request = match request_type {
            RequestType::SString => {
                let data = args[0];
                Request {
                    request_type: RequestType::SString,
                    raw: command,
                    data,
                }
            }
            //RequestType::Array => {}
            RequestType::BString => {
                let data = args[1];
                Request {
                    request_type: RequestType::BString,
                    raw: command,
                    data,
                }
            }
            RequestType::Unknown => Request {
                request_type: RequestType::Unknown,
                raw: b"",
                data: b"",
            },
        };
        Ok(request)
    }
}

impl RequestType {
    fn build(c: &u8) -> Self {
        match *c {
            b'+' => Self::SString,
            b'$' => Self::BString,
            //b'*' => RequestType::Array,
            _ => Self::Unknown,
        }
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                thread::spawn(|| {
                    let _ = handle_connection(stream);
                });
            }
            Err(e) => {
                println!("error: {e}");
            }
        }
    }
}

fn handle_connection(mut conn: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut buf = [0; 1024];
    loop {
        let n = conn.read(&mut buf)?;

        if n == 0 {
            break;
        }

        let request = Request::parse_command(&buf[0..=n])?;
        println!("{:?}", request);
        conn.write_all(b"+PONG\r\n")?;
    }
    Ok(())
}
