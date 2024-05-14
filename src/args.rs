use clap::Parser;
use clap::Subcommand;


#[derive(Debug, Parser)]
#[clap(name = "w")]
pub struct Args {
    #[clap(subcommand)]
    pub command: Option<Command>,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Inspect {
        path: String,
    },
	Display {
		path: String,
	}
}