// Uncomment this block to pass the first stage
use anyhow::Result;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;

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

fn handle_connection(mut conn: TcpStream) -> Result<()> {
    let mut buf = [0; 1024];
    loop {
        let n = conn.read(&mut buf)?;

        if n == 0 {
            break;
        }

        conn.write_all(b"+PONG\r\n")?;
    }
    Ok(())
}
