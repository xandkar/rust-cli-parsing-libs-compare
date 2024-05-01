use argh::FromArgs;

#[derive(FromArgs, Debug)]
/// Reach new heights.
struct Cli {
    /// whether or not to jump
    #[argh(switch, short = 'j')]
    jump: bool,

    /// how high to go
    #[argh(option)]
    height: usize,

    /// an optional nickname for the pilot
    #[argh(option)]
    pilot_nickname: Option<String>,
}

fn main() {
    let cli: Cli = argh::from_env();
    eprintln!("cli: {:?}", &cli);
}
