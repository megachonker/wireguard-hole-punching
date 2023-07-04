use env_logger::Env;
use std::thread;
use bincode::{deserialize, serialize};
use clap::Parser;
use log::{info, trace, error, debug, warn};
use std::{
    io::{Read, Write},
    net::{IpAddr, SocketAddr, TcpListener, TcpStream}, time::Duration,
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

        debug!("trying to connect to {} ...",ip_rdv);
        // let get_ip_stream =  CHK_ERROR!(TcpStream::connect_timeout(&SocketAddr::new(ip_rdv, 12345), Duration::from_secs(1)),"Failed to connect to the Rendezvous point");
        //Binding Socket reusable!
        let get_ip_stream = CHK_ERROR!(reuse_connect(SocketAddr::new(ip_rdv, 12345)),"error NET2 reuse socket bind");

        info!("Connected to the Rendezvous point!");

        debug!("First connection: {:?}",get_ip_stream);

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
    debug!("waiting RDV to get SOCKET");
    get_ip_stream.read(&mut buffer).unwrap();
    let received_socket: SocketAddr = CHK_ERROR!(deserialize(&buffer[..]),"SERD DESERIALIZE ERROR");
    debug!("Receive {:?} RDV",received_socket);


    info!("trying to connect back...");
    //create new socket
    let mut data_stream = TcpStream::connect(&received_socket).unwrap();

    //BAD
    // let mut data_stream = CHK_ERROR!(TcpStream::connect_timeout(received_socket),"Bad socket exchanged");
    // let mut data_stream = loop {
    //     match TcpStream::connect_timeout(&received_socket,Duration::from_millis(500)) {
    //         Ok(listener) => {
    //             info!("AVAIBLE");
    //             break listener;
    //         }
    //         Err(e) => {
    //             trace!("try to connect {}",e);
    //             // thread::sleep(Duration::from_secs(1));
    //         }
    //     }
    // };


    //pass the VPN
    info!("CONNECTED");

    //BAD
    data_stream.write_all(b"Hello, server !!!!!!!!!!!!!!!!!!!!!!").unwrap();

}

fn reuse_connect(addr:SocketAddr) -> Result<TcpStream, std::io::Error> {
    info!("connect FORCE {}",addr);
    let socket = TcpBuilder::new_v4()?
        .reuse_address(true)?
        .reuse_port(true)?
        .connect(addr)?;
    Ok(socket)
}

fn server(get_ip_stream: TcpStream){
    client(get_ip_stream);
    //BAD
    // let newsocket = force(get_ip_stream.local_addr().unwrap().to_string()).unwrap();
    
    // get_ip_stream.local_addr().unwrap().to_string();
    // let listener = loop {
        // match TcpListener::bind(get_ip_stream.local_addr().unwrap()) {
            // Ok(listener) => {
                // info!("AVAIBLE");
                // break listener;
            // }
            // Err(e) => {
                // trace!("waiting avaible lolu {}",e);
                // thread::sleep(Duration::from_secs(1));
            // }
        // }
    // };
    
    // debug!("Rebind du socket pour le client:");
    // let listener = CHK_ERROR!(TcpListener::bind(get_ip_stream.local_addr().unwrap()),"Failed to bind server Pub Hack socket");
    
    // let mut stream = CHK_ERROR!(TcpStream::connect(get_ip_stream.local_addr().unwrap()),"Failed to bind server Pub Hack socket");
    // debug!("Wait incoming ");
    // let mut stream = CHK_ERROR!(listener.incoming().next().unwrap(),"Erreur Ouverture du stream");
    // let mut buffer = [0; 1024];
    // stream.read(&mut buffer).unwrap();
    // warn!("RECEIVED DATA:{:?}",buffer);
}

fn handle_client(mut stream:TcpStream,server_socket:SocketAddr){
    let local_addr = stream.local_addr().unwrap();
    let peer_addr = stream.peer_addr().unwrap();

    info!("New connection:");
    debug!("Local address: {:?}", local_addr);
    debug!("randez_vous address: {:?}", peer_addr);
    debug!("server address: {:?}", server_socket);

    let buf = CHK_ERROR!(serialize(&server_socket),"SERD SERIALIZE ERROR");
    let slice_buf:&[u8] = &buf;
    debug!("SENDING Socket {:?} to {:?}",server_socket,stream.peer_addr());
    CHK_ERROR!(stream.write(slice_buf),"Sending error");

    let mut buf = [0; 0];
    trace!("bloquage du socket");
    
    match stream.read(&mut buf) {
        Ok(_) => info!("Data recus pandant l'atente {:?}",buf),
        Err(_) => warn!("un message a été recus dans le socket"),
    }
    
    
    // loop {
        // stream.set_read_timeout(None).expect("TIMEOUT ERR");
        // error!("ATTTTT");
        // stream.read(&mut buf).expect("socket closed prematurly");
        // error!("un message est passer sur le socket");
    // }
    // ;
    // trace!("Maintient du socket en vie 15 seconde");
    // thread::sleep(Duration::from_secs(15));
}

fn handle_server(stream:TcpStream,client_socket:SocketAddr){
    handle_client(stream,client_socket);
}


fn randezvous() {
    // Bind the TCP listener to the IP address and port
    let listener = CHK_ERROR!(TcpListener::bind("0.0.0.0:12345"),"Failed to bind Randez Vous");
    info!("waiting Connection:");
    let server_stream = listener.incoming().next().unwrap().unwrap();
    debug!("New 1/2 Connexion");
    let client_stream = listener.incoming().next().unwrap().unwrap();
    debug!("New 2/2 Connexion");

    //getting socket
    let server_socket = CHK_ERROR!(server_stream.peer_addr(),"Imposible d'avoir le socket");
    let client_socket = CHK_ERROR!(client_stream.peer_addr(),"Imposible d'avoir le socket");

    //start 2 thread
    let handler = thread::spawn(move|| {handle_client(client_stream,server_socket)});
    handle_server(server_stream,client_socket);
    handler.join().unwrap();
    info!("fin randez vous...");
}