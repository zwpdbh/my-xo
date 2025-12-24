use crate::error::{Error, Result};
use image::{ImageError, ImageFormat};

use scap::{
    capturer::{Area, Capturer, Options, Point, Size},
    frame::Frame,
};
use std::fs;
use std::io;
use std::path::PathBuf;

pub struct ScreenshotConfig {
    pub width: u32,
    pub height: u32,
    pub x: u32,
    pub y: u32,
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
    // Check if the platform is supported
    if !scap::is_supported() {
        return Err(Error::PlatformNotSupported);
    }

    // Check if we have permission to capture screen
    // If we don't, request it.
    if !scap::has_permission() {
        if !scap::request_permission() {
            return Err(Error::ScreenshotError(
                "Permission to capture screen denied".to_string(),
            ));
        }
    }

    // Get recording targets
    let targets = scap::get_all_targets();
    println!("Targets: {:?}", targets);

    // All your displays and windows are targets
    // You can filter this and capture the one you need.

    // Create Options
    let options = Options {
        fps: 60,
        target: None, // None captures the primary display
        show_cursor: true,
        show_highlight: true,
        excluded_targets: None,
        output_type: scap::frame::FrameType::BGRAFrame,
        output_resolution: scap::capturer::Resolution::_720p,
        crop_area: Some(Area {
            origin: Point { x: 0.0, y: 0.0 },
            size: Size {
                width: 2000.0,
                height: 1000.0,
            },
        }),
        ..Default::default()
    };

    // Create Capturer
    let mut capturer = Capturer::build(options).unwrap();

    // Start Capture
    capturer.start_capture();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    // Stop Capture
    capturer.stop_capture();

    todo!()
}

fn convert_frame_to_image(frame: &Frame) -> Result<image::RgbaImage> {
    todo!()
}
