use std::io;
use std::io::TcpStream;

fn main() {
    println!("Guess the number!");

    let mut guesses: int = 0;
    let mut reader = io::stdin();

    let mut stream = TcpStream::connect("127.0.0.1", 8012);

    loop {
        print!("Please input guess #{}: ", guesses + 1);

        let input = reader.read_line().ok().expect("Failed to read line");
        let input_num: Option<u8> = from_str(input.as_slice().trim());

        let num = match input_num  {
            Some(num) => num,
            None      => {
                println!("Please input a number.");
                continue;
            }
        };

        println!("You guessed: {}", num);
        guesses += 1;


        match stream.write([num]) {
            Err(e) => { 
                println!("Error: {}", e); 
                break;
            },
            Ok(_) => {
                let mut buf: [u8, ..1] = [0];

                match stream.read(buf) {
                    Err(e) => { 
                        println!("Error: {}", e); 
                        break;
                    },
                    Ok(_) => {},
                }
                
                match buf[0] {
                    0 => println!("Too small!"),
                    1 => println!("Too big!"),
                    2 => { println!("You win!"); break },
                    _ => { println!("Oh dear, something bad happened."); break; },
                }
            },
        }
    }


    println!("You took {} guesses!", guesses);
}

