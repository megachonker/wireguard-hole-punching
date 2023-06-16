use bincode::{deserialize, serialize};
use tokio::task;
use clap::Parser;
use std::thread;
use std::{
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
};
#[derive(Parser)] // requires `derive` feature
struct Cli {
    /// Mode client: besoin addresse rendezvous
    #[arg(short, long)]
    client_flag: bool,

    /// Mode server: besoin addresse rendezvous
    #[arg(short, long)]
    server_flag: bool,

    /// Mode rendezvous: besoin de rien
    #[arg(short, long)]
    rdv_flag: bool,

    /// Addresse ip du point de rendez vous
    rdv_address: Option<IpAddr>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct IpPort {
    ip: IpAddr,
    port: u16,
}

fn main() {
    let args = Cli::parse();
    let ip_rdv: IpAddr;

    // do i have randezvous ?
    if !args.rdv_flag {
        if !(args.client_flag ^ args.server_flag) {
            println!("flag client ^ server");
            return;
        }
        match args.rdv_address {
            Some(res) => ip_rdv = res,
            None => {
                println!("Give IP address Please");
                return;
            }
        }
        let get_ip_stream = TcpStream::connect(SocketAddr::new(ip_rdv, 12345)).unwrap();

        //I am client
        if args.client_flag {
            client(get_ip_stream);
        }
        //I am server
        else if args.server_flag {
            server(get_ip_stream);
        }
        // I am nothing
        else {
            println!("no flag");
        }
    }
    // I am randezvous
    else {
        randezvous();
    }
}

fn client(mut get_ip_stream: TcpStream) {
    //listen to receive a IpPort
    let mut buffer = [0; 1024];
    get_ip_stream.read(&mut buffer).unwrap();
    let received_ip_port: IpPort = deserialize(&buffer[..]).unwrap();

    //create new socket
    let mut data_stream =
        TcpStream::connect(SocketAddr::new(received_ip_port.ip, received_ip_port.port)).unwrap();
    //pass the VPN
    data_stream.write_all(b"Hello, server!").unwrap();
}

fn server(get_ip_stream: TcpStream) {
    // println!("server");
    client(get_ip_stream)
}

fn handle_connection(bind:TcpListener){
    // match bind.incoming() {
    //     Ok(strem) => {},
    // }
    // println!("New: {:?}",stream);
}

fn randezvous() {
    // Bind the TCP listener to the IP address and port
    // let listener = TcpListener::bind("127.0.0.1:1234").expect("Failed to bind to address");
    // listener.incoming().aw
    // // Accept incoming connections in a loop
    // for stream in listener.incoming() {
    //     match stream {
    //         Ok(stream) => {
    //             // Handle each incoming connection in a separate thread
    //             thread::spawn(move || {
    //                 handle_connection(stream);
    //             });
    //         }
    //         Err(e) => {
    //             println!("Error accepting connection: {}", e);
    //         }
    //     }
    // }

    println!("randezvous");
}
