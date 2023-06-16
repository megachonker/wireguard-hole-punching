use clap::Parser;
use std::{net::{IpAddr,SocketAddr,TcpStream,Ipv4Addr}, io::{Read, Write}};
use bincode::{serialize, deserialize};
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
struct IpPort{
    ip:IpAddr,
    port:u16,
}

fn main() {
    let args = Cli::parse();
    let ip_rdv: IpAddr;
    
    // do i have randezvous ?
    if !args.rdv_flag {
        if  !(args.client_flag ^ args.server_flag){
            println!("flag client ^ server"); return;
        }
        match args.rdv_address {
            Some(res) => ip_rdv = res,
            None => {println!("Give IP address Please"); return;},
        }
        let mut get_ip_stream = TcpStream::connect(SocketAddr::new(ip_rdv, 12345)).unwrap();
        
        //listen to receive a IpPort
        let mut buffer = [0; 1024];
        get_ip_stream.read(&mut buffer).unwrap();
        get_ip_stream.read(&mut buffer);
        let received_ip_port: IpPort = deserialize(&buffer[..]).unwrap();
        let mut data_stream = TcpStream::connect(SocketAddr::new(received_ip_port.ip, received_ip_port.port)).unwrap();
        
        let mut buffer = [0; 1024];
        data_stream.write_all(b"Hello, server!").unwrap();


        // //I am client
        // if args.client_flag {
        //     client(ip_rdv,port);
        // } 
        // //I am server
        // else if args.server_flag {
        //     server(ip_rdv,port)
        // }
        //I am nothing
        // else {
        //     println!("no flag")
        // }
    }
    // I am randezvous 
    else {
        randezvous();
    }
}

fn client(socket:SocketAddr) {
    // println!("client");
    // println!("{:?}", ip_rdv);
}

fn server(socket:SocketAddr) {
    // println!("server");
    // println!("{:?}", ip_rdv);
}

fn randezvous() {
    println!("randezvous");
}
