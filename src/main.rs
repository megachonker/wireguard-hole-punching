use bincode::{deserialize, serialize};
use clap::Parser;
use env_logger::Env;
use log::{debug, error, info, trace, warn};
use std::alloc::System;
use std::thread::{self, Thread};
use std::{
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream},
    time::Duration,
};

use net2::{unix::UnixTcpBuilderExt, TcpBuilder};

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

        debug!("trying to connect to {} ...", ip_rdv);
        //Binding Socket reusable!
        let get_ip_stream = CHK_ERROR!(
            reuse_connect(SocketAddr::new(ip_rdv, 12345)),
            "error NET2 reuse socket bind"
        );
        info!("Connected to the Rendezvous point!");

        debug!("First connection: {:?}", get_ip_stream);

        if args.client_flag {
            trace!("I am client");
            client(get_ip_stream);
        } else {
            trace!("I am server");
            server(get_ip_stream);
        }
    } else {
        info!("I am randezvous");
        randezvous();
    }
}

fn client(mut get_ip_stream: TcpStream) {
    //listen to receive a IpPort
    let mut buffer = [0; 1024];
    debug!("waiting RDV to get SOCKET");
    get_ip_stream.read(&mut buffer).unwrap();
    let received_socket: SocketAddr =
        CHK_ERROR!(deserialize(&buffer[..]), "SERD DESERIALIZE ERROR");
    debug!("Receive {:?} RDV", received_socket);

    let src_socket = get_ip_stream.local_addr().expect("cannoc get locall addr");
    //maintain alive by receving
    let th = thread::spawn(move || loop {
        let mut buffer = [0; 1];
        if get_ip_stream.read(&mut buffer).unwrap() != 0 {
            trace!("HEARTHBEAT {:?}", buffer);
            CHK_ERROR!(get_ip_stream.write(&buffer),"err envoit ack");
        } else {
            trace!("RDV socket closed");
            std::process::exit(1);
        }
    });

    //bind
    thread::spawn(move || loop {
        let azer = TcpBuilder::new_v4().unwrap().reuse_address(true).unwrap().reuse_port(true).unwrap().listen(2).unwrap();
        info!("!!{:?}",azer.accept().unwrap());
        info!("Connexion ACCEPTED!");
        std::process::exit(11);
    });

    info!("trying to connect back...");
    //create new socket
    // TcpStream::connect_timeout(&received_socket,Duration::from_millis(500)

    loop {
        match connect_from_to(
            src_socket,
            received_socket,
        ) {
            Ok(mut data_stream) => {
                //pass the VPN
                info!("!!!!!!!!!!!!!!CONNECTED!!!!!!!!!!!!!!");
                data_stream
                    .write_all(b"Hello, server !!!!!!!!!!!!!!!!!!!!!!")
                    .unwrap();
                std::process::exit(11);
            }
            Err(_) => warn!("retrying connect"),
        }
        thread::sleep(Duration::from_millis(750));
    }
    th.join().unwrap();
}

fn reuse_connect(addr: SocketAddr) -> Result<TcpStream, std::io::Error> {
    trace!("connect FORCE {}", addr);
    let socket = TcpBuilder::new_v4()?
        .reuse_address(true)?
        .reuse_port(true)?
        .connect(addr)?;
    Ok(socket)
}
fn connect_from_to(source: SocketAddr, dest: SocketAddr) -> Result<TcpStream, std::io::Error> {
    trace!("connect_from_to {}=>{}", source, dest);
    let socket = TcpBuilder::new_v4()?
        .reuse_address(true)?
        .reuse_port(true)?
        .bind(source)?
        .connect(dest)?;
    Ok(socket)
}

fn server(get_ip_stream: TcpStream) {
    client(get_ip_stream);
}

fn handle_client(mut stream: TcpStream, server_socket: SocketAddr) {
    let local_addr = stream.local_addr().unwrap();
    let peer_addr = stream.peer_addr().unwrap();

    info!("New connection:");
    debug!("Local address: {:?}", local_addr);
    debug!("randez_vous address: {:?}", peer_addr);
    debug!("server address: {:?}", server_socket);

    let buf = CHK_ERROR!(serialize(&server_socket), "SERD SERIALIZE ERROR");
    let slice_buf: &[u8] = &buf;
    debug!(
        "SENDING Socket {:?} to {:?}",
        server_socket,
        stream.peer_addr()
    );
    CHK_ERROR!(stream.write(slice_buf), "Sending error");

    let mut buf: [u8; 1] = [0];
    trace!("bloquage du socket");

    //maintain allive
    loop {
        CHK_ERROR!(stream.write(&buf), "Sending error");
        trace!("HEART sended");
        CHK_ERROR!(stream.read(&mut buf), "recv ACK");
        thread::sleep(Duration::from_secs(1));
    }
}

fn handle_server(stream: TcpStream, client_socket: SocketAddr) {
    handle_client(stream, client_socket);
}

fn randezvous() {
    // Bind the TCP listener to the IP address and port
    let listener = CHK_ERROR!(
        TcpListener::bind("0.0.0.0:12345"),
        "Failed to bind Randez Vous"
    );
    info!("waiting Connection:");
    let server_stream = listener.incoming().next().unwrap().unwrap();
    debug!("New 1/2 Connexion");
    let client_stream = listener.incoming().next().unwrap().unwrap();
    debug!("New 2/2 Connexion");

    //getting socket
    let server_socket = CHK_ERROR!(server_stream.peer_addr(), "Imposible d'avoir le socket");
    let client_socket = CHK_ERROR!(client_stream.peer_addr(), "Imposible d'avoir le socket");

    //start 2 thread
    let handler = thread::spawn(move || handle_client(client_stream, server_socket));
    handle_server(server_stream, client_socket);
    handler.join().unwrap();
    info!("fin randez vous...");
}
