// Uncomment this block to pass the first stage
use anyhow::Result;
use std::io::Write;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    //
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("accepted new connection");
                let _ = handle_connection(stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut conn: TcpStream) -> Result<()> {
    conn.write_all(b"+PONG\r\n")?;
    Ok(())
}
