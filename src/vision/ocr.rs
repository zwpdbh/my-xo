use crate::{Error, Result};
use rusty_tesseract::{Args, Image};
use std::{collections::HashMap, path::Path};
use tracing::info;

pub fn run_ocr(image_path: &str) -> Result<()> {
    if Path::new(image_path).exists() == false {
        return Err(Error::ImageError(format!(
            "Image path does not exist: {}",
            image_path
        )));
    }

    let img = Image::from_path(image_path).map_err(|e| Error::ImageError(e.to_string()))?;

    let my_args = Args {
        lang: "eng".to_string(),
        config_variables: HashMap::from([(
            "tessedit_char_whitelist".into(),
            "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".into(),
        )]),
        dpi: Some(150),
        psm: Some(6),
        oem: Some(3),
    };

    let output = rusty_tesseract::image_to_string(&img, &my_args)
        .map_err(|e| Error::ImageError(e.to_string()));

    info!("The String output is: {:?}", output);

    Ok(())
}
