mod error;
use clap::Parser;
pub use error::*;

mod cmd;
use cmd::{Args, Command};

mod tracer;
use tracer::setup_simple_tracing;

mod vision;
use tracing::{error, info};
use vision::*;

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
                Ok(path) => info!("Screenshot successfully saved to: {}", path.display()),
                Err(e) => error!("Error taking screenshot: {:?}", e),
            }
        }
        Command::Ocr { image_path } => run_ocr(&image_path).unwrap(),
    }
}
