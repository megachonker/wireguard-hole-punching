use clap::Parser;
use std::net::IpAddr;
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
    let args = Cli::parse();
    let ip_rdv: IpAddr;

    // do i have randezvous ?
    if !args.rdv_flag {
        match args.rdv_address {
            Some(res) => ip_rdv = res,
            None => {println!("Give IP address Please"); return;},
        }
        //I am client
        if args.client_flag {
            client(ip_rdv);
        } 
        //I am server
        else if args.server_flag {
            server(ip_rdv)
        }
        //I am nothing
        else {
            println!("no flag")
        }
    }
    // I am randezvous 
    else {
        randezvous();
    }
}

fn client(ip_rdv: IpAddr) {
    println!("client");
    println!("{:?}", ip_rdv);
}

fn server(ip_rdv: IpAddr) {
    println!("server");
    println!("{:?}", ip_rdv);
}

fn randezvous() {
    println!("randezvous");
}
