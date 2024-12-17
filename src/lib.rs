use qrcode::{QrCode, QrResult};
use image::LumaA;

// use qrcode::render::{svg, string, unicode, pic};

#[deprecated = "I'll Implement This Later."]
pub enum QRMode {
    SVG,
    String,
    Unicode,
    PIC,
    Image
}

#[derive(Debug)]
struct QRData {
    content: String,
    // mode: Option<QRMode> I'll Implement This Later.
}

impl QRData {
    pub fn new(content: &String, /*mode: Option<QRMode>*/) -> QRData {
        QRData {
            content: content.to_owned(),
            // mode: mode.or_else(|| { Some(QRMode::Image) })
        }
    }

    pub fn form_image<'a>(data: &'a QRData, img_path: &'a str) -> QrResult<&'a str> {
        let code = QrCode::new(data.content.as_bytes());
        if code.is_err() {
            panic!("Something went wrong while creating the QrCode.");
        }

        let render = code.unwrap().render::<LumaA<u8>>().build();

        let save = render.save_with_format(img_path, image::ImageFormat::Png);

        if save.is_err() {
            panic!("Something went wrong while saving the QrCode.");
        }

        Ok(img_path)
    }
}

impl Default for QRData {
    fn default() -> Self {
        QRData { content: "https://www.google.com".to_owned() }
    }
}