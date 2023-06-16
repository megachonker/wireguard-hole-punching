use clap::Parser;

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Mode client: besoin addresse rendezvous 
    #[arg(short,long)]
    client_flag: bool,

    /// Mode server: besoin addresse rendezvous
    #[arg(short,long)]
    server_flag: bool,

    /// Mode rendezvous: besoin de rien
    #[arg(short,long)]
    rdv_flag: bool,

    /// Addresse ip du point de rendez vous
    rdv_address: Option<std::net::IpAddr>,
}

fn main() {
    let args = Cli::parse();
}