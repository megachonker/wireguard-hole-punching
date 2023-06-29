use env_logger::Env;
use bincode::{deserialize};
use clap::Parser;
use log::{info, trace, error, debug, warn};
use std::{
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream}, time::Duration,
};


macro_rules! CHK_ERROR {
    ($result:expr, $message:expr) => {
        match $result {
            Ok(value) => value,
            Err(err) => {
                error!("{}: {}", $message, err);
                std::process::exit(1);
            }
        }
    };
}



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
    env_logger::Builder::from_env(Env::default().default_filter_or("trace")).init();
    let args = Cli::parse();
    let ip_rdv: IpAddr;
    
    // do i have randezvous ?
    if !args.rdv_flag {
        if !(args.client_flag ^ args.server_flag) {
            error!("No Server or Client flag set !");
            return;
        }
        match args.rdv_address {
            Some(res) => ip_rdv = res,
            None => {
                error!("Give IP address Please");
                return;
            }
        }

        debug!("trying to connect to {} ...",ip_rdv);
        let get_ip_stream =  CHK_ERROR!(TcpStream::connect_timeout(&SocketAddr::new(ip_rdv, 12345), Duration::from_secs(1)),"Failed to connect to the Rendezvous point");
        info!("Connected to the Rendezvous point!");

        warn!("connection information: {:?}",get_ip_stream);

        if args.client_flag {
            trace!("I am client");
            client(get_ip_stream);
        } else {
            trace!("I am server");
            server(get_ip_stream);
        }

    }
    else {
        info!("I am randezvous");
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

    info!("New connection:");
    debug!("Local address: {:?}", local_addr);
    debug!("Remote address: {:?}", peer_addr);
}

fn handle_server(stream:TcpStream){
    handle_client(stream);
}


fn randezvous() {
    // Bind the TCP listener to the IP address and port
    let listener = CHK_ERROR!(TcpListener::bind("0.0.0.0:12345"),"Failed to bind Randez Vous");
    info!("waiting Connection:");
    let server_stream = listener.incoming().next().unwrap().unwrap();
    debug!("New Connexion");
    let client_stream = listener.incoming().next().unwrap().unwrap();
    debug!("New Connexion");

    rayon::scope(|s| {
        s.spawn(|_| handle_client(server_stream));
        s.spawn(|_| handle_server(client_stream));
    });

    println!("randezvous");
}