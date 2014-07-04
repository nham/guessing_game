use std::io::{TcpListener, TcpStream};
use std::io::{Acceptor, Listener};
use std::io::{IoResult};
use std::rand::Rng;

fn main() {
    let listener: IoResult<TcpListener> = TcpListener::bind("127.0.0.1", 8012);
    let mut acceptor = listener.listen();

    fn handle_client(mut stream: TcpStream) {
        let mut buf: [u8, ..1] = [0];
        let secret_number = std::rand::task_rng().gen_range(0u8, 255);
        println!("Secret number is {}", secret_number);

        loop {
            match stream.read(buf) {
                Err(e) => { 
                    println!("Error: {}", e); 
                    break;
                },
                Ok(_) => {},
            }
            
            let guess = buf[0];
            println!("Received guess: {}", guess);

            let response = match guess.cmp(&secret_number) {
                Less    => 0,
                Greater => 1,
                Equal   => 2,
            };

            match stream.write([response]) {
                Err(e) => { 
                    println!("Error: {}", e); 
                    break;
                },
                Ok(_) => {
                    if response == 2 {
                        println!("Drat, they guessed it.");
                        break;
                    }
                }
            }
        }
    }

    for stream in acceptor.incoming() {
        match stream {
            Err(e) => fail!("Error: {}", e),
            Ok(stream) => spawn(proc() {
                handle_client(stream)
            }),
        }
    }

    drop(acceptor);
}
