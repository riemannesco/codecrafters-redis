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
    data: String,
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

fn parse_command<'a>(command: &'a [u8]) -> Result<Request<'a>, Box<dyn std::error::Error>> {
    let request_type = RequestType::build(command.first().unwrap());
    if request_type.eq(&RequestType::Unknown) {
        println!("unknown");
        return Err("unknown request type".into());
    }
    let args: Vec<&[u8]> = command
        .split(|&c| c == b'\n')
        .map(|line| line.strip_suffix(b"\r").unwrap_or(line))
        .collect();
    let request: Request = match request_type {
        RequestType::SString => {
            println!("request parsed");
            let data = String::from_utf8(args[0].to_vec())?;
            Request {
                request_type: RequestType::SString,
                raw: command,
                data,
            }
        }
        //RequestType::Array => {}
        RequestType::BString => {
            println!("requesssst parsed");
            let size: u32 = String::from_utf8(args[0].to_vec())?.parse()?;
            let data = String::from_utf8(args[1].to_vec())?;
            Request {
                request_type: RequestType::BString,
                raw: command,
                data,
            }
        }
        RequestType::Unknown => Request {
            request_type: RequestType::Unknown,
            raw: b"",
            data: "".into(),
        },
    };
    println!("{:?}", request.data);
    return Ok(request);
}

impl RequestType {
    fn build(c: &u8) -> RequestType {
        match *c {
            b'+' => RequestType::SString,
            b'$' => RequestType::BString,
            //b'*' => RequestType::Array,
            _ => RequestType::Unknown,
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
                println!("error: {}", e);
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

        let request = parse_command(&mut buf[0..=n])?;
        println!("request parsed");
        conn.write_all(b"+PONG\r\n")?;
    }
    Ok(())
}
