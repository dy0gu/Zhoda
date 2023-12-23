use clap::{command, Arg, ArgMatches};
use ctrlc::set_handler;
use std::io::{self, stdin, Write};
use std::net::TcpStream;
use std::process::exit;

const ADDRESS_ARG: &str = "address";
const DEBUG_ARG: &str = "debug";

fn main() {
    let args = arg_parse();

    // Install Ctrl+C handler to gracefully close
    match set_handler(move || {
        exit(0);
    }) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    }

    // Get adress from CLI arguments
    let address = args.get_one::<String>(ADDRESS_ARG).unwrap();

    // Connect to the server
    let mut stream = match TcpStream::connect(address) {
        Ok(stream) => {
            println!("User input being sent to {}", address);
            stream
        }
        Err(e) => match e.kind() {
            io::ErrorKind::InvalidInput => {
                eprintln!("Invalid address entered");
                exit(1);
            }
            _ => {
                eprintln!("{}", e);
                exit(1);
            }
        },
    };

    // Continuously send user input to the server
    loop {
        let mut input = String::new();

        stdin().read_line(&mut input).unwrap();

        match stream.write_all(input.as_bytes()) {
            Ok(_) => {}
            Err(e) => match e.kind() {
                io::ErrorKind::ConnectionReset => {
                    eprintln!("Connection to server lost");
                    exit(1);
                }
                _ => {
                    eprintln!("{}", e);
                    exit(1);
                }
            },
        }

        stream.flush().unwrap();
    }
}

fn arg_parse() -> ArgMatches {
    command!()
        .arg(
            Arg::new(ADDRESS_ARG)
                .required(true)
                .help("Socket address to connect to"),
        )
        .arg(
            Arg::new(DEBUG_ARG)
                .short('d')
                .long("debug")
                .num_args(0)
                .help("Print debug information"),
        )
        .get_matches()
}
