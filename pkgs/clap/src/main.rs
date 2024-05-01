use clap::Parser;

#[derive(Parser, Debug)]
/// Reach new heights.
struct Cli {
    /// whether or not to jump
    #[clap(short = 'j')]
    jump: bool,

    /// how high to go
    #[clap(long)]
    height: usize,

    /// an optional nickname for the pilot
    #[clap(long)]
    pilot_nickname: Option<String>,
}

fn main() {
    let cli = Cli::parse();
    eprintln!("cli: {:?}", &cli);
}
