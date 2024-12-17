mod qr_data;

use std::env;

use qr_data::QRData;
use colored::Colorize;

fn main() {
    loop {
        println!("{}", "Enter some content to turn it into a QR Code: ".green());

        let args: Vec<String> = env::args().collect();
        if args.len() != 2 {
            eprintln!("{}", "You must enter some content and image path.!".red())
        }

        let qr_code = QRData::new(&args[1]);
        let image = QRData::form_image(&qr_code, &args[2]);

        println!("{}", format!("Created your Qr Image: {}", image.unwrap()))     
    }
}
