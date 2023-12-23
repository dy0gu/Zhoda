use clap::{command, Arg, ArgMatches};
use ctrlc::set_handler;
use std::io::{self, Read};
use std::net::{TcpListener, TcpStream};
use std::process::exit;
use std::thread;

const PORT_ARG: &str = "port";
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

    // Get port from CLI arguments, let OS choose if not manually set
    let port = match args.contains_id(PORT_ARG) {
        true => args.get_one::<String>(PORT_ARG).unwrap(),
        false => "0",
    };

    // Build socket address from port
    let address = format!("127.0.0.1:{}", port);

    // Start listening by binding to the address
    let listener = match TcpListener::bind(address) {
        Ok(listener) => {
            println!(
                "Listening for connections on port {}",
                listener.local_addr().unwrap().port()
            );
            listener
        }
        Err(e) => match e.kind() {
            io::ErrorKind::InvalidInput => {
                eprintln!("\nInvalid port entered");
                exit(1);
            }
            io::ErrorKind::AddrInUse => {
                eprintln!("\nPort already in use");
                exit(1);
            }
            _ => {
                eprintln!("{}", e);
                exit(1);
            }
        },
    };

    // Keep track of each specific client by their connection order
    let mut id = 0;

    // Accept connections and spawn a thread to handle each one
    for stream in listener.incoming() {
        id += 1;
        match stream {
            Ok(stream) => {
                println!("New client connected with ID {}", id);
                thread::spawn(move || {
                    handle_client(stream, id);
                });
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        }
    }

    exit(0);
}

fn arg_parse() -> ArgMatches {
    command!()
        .arg(Arg::new(PORT_ARG).help("Set port to listen on manually"))
        .arg(
            Arg::new(DEBUG_ARG)
                .short('d')
                .long("debug")
                .num_args(0)
                .help("Print debug information"),
        )
        .get_matches()
}

// Continuously read from the client stream and print the received messages
fn handle_client(mut stream: TcpStream, id: u32) {
    let mut buffer = String::new();
    loop {
        match stream.read_to_string(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    break;
                }
                println!("Received from client {}: {}", id, buffer);
                buffer.clear();
            }
            Err(e) => match e.kind() {
                io::ErrorKind::ConnectionReset => {
                    println!("Connection to client {} lost", id);
                    break;
                }
                _ => {
                    eprintln!("{}", e);
                    break;
                }
            },
        }
    }
}
