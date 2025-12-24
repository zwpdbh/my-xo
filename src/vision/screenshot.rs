use crate::error::Result;
use screenshots::Screen;
use std::path::PathBuf;
use std::time::Instant;

pub struct ScreenshotConfig {
    pub width: u32,
    pub height: u32,
    pub x: i32,
    pub y: i32,
}

impl Default for ScreenshotConfig {
    fn default() -> Self {
        Self {
            width: 400,
            height: 300,
            x: 0,
            y: 0,
        }
    }
}

pub fn capture_screenshot(config: ScreenshotConfig) -> Result<PathBuf> {
    let start = Instant::now();
    let screens = Screen::all().unwrap();

    let capture_path = PathBuf::from("target/capture_display_with_point.png");

    for screen in screens {
        println!("capturer {screen:?}");
        let mut image = screen.capture().unwrap();

        image
            .save(format!("target/{}.png", screen.display_info.id))
            .unwrap();

        image = screen
            .capture_area(config.x, config.y, config.width, config.height)
            .unwrap();
        image
            .save(format!("target/{}-2.png", screen.display_info.id))
            .unwrap();
    }

    let screen = Screen::from_point(100, 100).unwrap();
    println!("capturer {screen:?}");

    let image = screen.capture_area(300, 300, 300, 300).unwrap();
    image.save(&capture_path).unwrap();
    println!("运行耗时: {:?}", start.elapsed());

    Ok(capture_path)
}
