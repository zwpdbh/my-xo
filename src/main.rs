mod error;
use clap::Parser;
pub use error::*;

mod cmd;
use cmd::{Args, Command};

mod tracer;
use tracer::setup_simple_tracing;

mod vision;
use vision::screenshot::{ScreenshotConfig, capture_screenshot};

fn main() {
    setup_simple_tracing();

    let args = Args::parse();
    match args.cmd {
        Command::Screenshot {
            width,
            height,
            x,
            y,
        } => {
            let config = ScreenshotConfig {
                width,
                height,
                x,
                y,
            };

            match capture_screenshot(config) {
                Ok(path) => println!("Screenshot successfully saved to: {}", path.display()),
                Err(e) => eprintln!("Error taking screenshot: {:?}", e),
            }
        }
    }
}
