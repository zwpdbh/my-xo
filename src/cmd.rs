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
        #[arg(long = "width", default_value = "500")]
        width: u32,

        #[arg(long = "height", default_value = "800")]
        height: u32,

        /// X coordinate of the top-left corner (default: 0)
        #[arg(short = 'x', long = "x", default_value = "0")]
        x: i32,

        /// Y coordinate of the top-left corner (default: 0)
        #[arg(short = 'y', long = "y", default_value = "0")]
        y: i32,
    },
    Ocr {
        #[arg(long = "image-path", default_value = "target/screenshot.png")]
        image_path: String,
    },
}
