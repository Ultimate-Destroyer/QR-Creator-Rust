mod qr_data;

use std::io::{self, Write};
use qr_data::QRData;
use colored::Colorize;

fn main() {
    loop {
        // Prompt the user for input
        print!("{}", "Enter some content to turn it into a QR Code (type 'exit' to quit): ".green());
        io::stdout().flush().unwrap();

        let mut content = String::new();
        io::stdin().read_line(&mut content).expect("Failed to read line");

        let content = content.trim();

        if content.eq_ignore_ascii_case("exit") {
            println!("{}", "Exiting...".yellow());
            break;
        }

        print!("{}", "Enter the image path: ".green());
        io::stdout().flush().unwrap();

        let mut image_path = String::new();
        io::stdin().read_line(&mut image_path).expect("Failed to read line");

        let image_path = image_path.trim();

        // Create QR code and image
        let qr_code = QRData::new(&content.to_owned());
        
        match QRData::form_image(&qr_code, image_path) {
            Ok(image) => println!("{}", format!("Created your QR Image: {}", image)),
            Err(e) => eprintln!("{}", format!("Error creating QR Image: {}", e).red()),
        }
    }
}
