use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author = "zhaowei", version, about)]
pub struct Args {
    #[clap(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand, Debug, Clone)]
pub enum Command {
    Screenshot {
        /// Width of the screenshot area (default: 400)
        #[arg(short = 'w', long = "width", default_value = "400")]
        width: u32,

        /// Height of the screenshot area (default: 300)
        #[arg(short = 'h', long = "height", default_value = "300")]
        height: u32,

        /// X coordinate of the top-left corner (default: 0)
        #[arg(short = 'x', long = "x", default_value = "0")]
        x: u32,

        /// Y coordinate of the top-left corner (default: 0)
        #[arg(short = 'y', long = "y", default_value = "0")]
        y: u32,
    },
}
