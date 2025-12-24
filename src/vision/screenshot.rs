use crate::error::Result;

use screenshots::Screen;
use screenshots::image::{ImageBuffer, Rgba};
use std::path::PathBuf;
use std::time::Instant;
use tracing::info;

pub struct ScreenshotConfig {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
}

pub fn capture_screen(
    screen: &Screen,
    config: &ScreenshotConfig,
) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
    let image = screen
        .capture_area(config.x, config.y, config.width, config.height)
        .unwrap();

    Ok(image)
}

pub fn capture_screenshot(config: ScreenshotConfig) -> Result<PathBuf> {
    let start = Instant::now();
    let screen = Screen::all().unwrap()[0];

    let capture_path = PathBuf::from("target/screenshot_with_area.png");
    let captured_screenshot = capture_screen(&screen, &config)?;
    captured_screenshot.save(&capture_path).unwrap();

    info!("elapsed time: {:?}", start.elapsed());
    Ok(capture_path)
}
