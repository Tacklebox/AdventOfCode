use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long)]
    pub debug: bool,

    /// Which part to run
    #[arg(short, long)]
    pub part: u8,
}

impl Cli {
    pub fn from_args() -> Self {
        Self::parse()
    }
}
