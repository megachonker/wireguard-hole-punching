use bincode::{deserialize, serialize};
use clap::Parser;
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

fn handle_client(stream:TcpStream){
    let local_addr = stream.local_addr().unwrap();
    let peer_addr = stream.peer_addr().unwrap();

    println!("New connection:");
    println!("Local address: {:?}", local_addr);
    println!("Remote address: {:?}", peer_addr);
}

fn handle_server(stream:TcpStream){
    handle_client(stream);
}


fn randezvous() {
    // Bind the TCP listener to the IP address and port
    let listener = TcpListener::bind("127.0.0.1:1234").expect("Failed to bind to address");
    let server_stream = listener.incoming().next().unwrap().unwrap();
    let client_stream = listener.incoming().next().unwrap().unwrap();

    rayon::scope(|s| {
        s.spawn(|_| handle_client(server_stream));
        s.spawn(|_| handle_server(client_stream));
    });

    println!("randezvous");
}