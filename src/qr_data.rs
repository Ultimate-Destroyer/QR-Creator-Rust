use qrcode::{QrCode, QrResult};
use image::{LumaA, error::ImageError};
use std::fs;

#[allow(dead_code)]
#[deprecated = "I'll Implement This Later."]
pub enum QRMode {
    SVG,
    String,
    Unicode,
    PIC,
    Image,
}

#[derive(Debug)]
pub struct QRData {
    content: String,
}

impl QRData {
    pub fn new(content: &String) -> QRData {
        QRData {
            content: content.to_owned(),
        }
    }

    pub fn form_image<'a>(data: &'a QRData, img_path: &'a str) -> QrResult<&'a str> {
        let code = QrCode::new(data.content.as_bytes())?;

        let render = code.render::<LumaA<u8>>().build();

        let dir = std::path::Path::new(img_path).parent().unwrap();
        if !dir.exists() {
            _ = fs::create_dir_all(dir).map_err(|e| ImageError::from(e));
        }

        _ = render.save_with_format(img_path, image::ImageFormat::Png)
            .map_err(|e: ImageError| ImageError::from(e));

        Ok(img_path)
    }
}

impl Default for QRData {
    fn default() -> Self {
        QRData { content: "https://www.google.com".to_owned() }
    }
}
