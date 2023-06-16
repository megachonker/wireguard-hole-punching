use std::io;
use clap::Parser;
use clap_complete::{generate, shells::{Bash,Fish,Zsh}};

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

    // /// Génère l'autocompletion bash
    // #[arg(long)]
    // generate_completion:Option<>
    // generate_completion:bool

}

// #[derive(Clone)]
// enum opt_shell {
//     bash,
//     zsh,
//     fish
// }

fn main() {
    let args = Cli::parse();
    // if args.generate_completion {
        // generate(Bash, &mut cli::build_cli(), "myapp", &mut io::stdout());
    // }
}