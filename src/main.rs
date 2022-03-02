use clap::Parser;
use std::io::Write;
use viuer::{print_from_file, Config};

// Simple CLI to make QR codes
#[derive(Parser, Debug)]
struct Cli {
    #[clap(short = 'd', long = "data")]
    // The information contained by the QR code
    data: String,

    #[clap(short = 'o', long = "output", default_value = ".")]
    #[clap(parse(from_os_str))]
    output: std::path::PathBuf,

    #[clap(short = 's')]
    #[clap(long = "show")]
    display: bool,
}


fn main() {
    let args = Cli::parse();
    
    // Using blocking request
    let res = reqwest::blocking::get(format!("https://api.dhravya.me/qrcode?query={}", args.data).as_str());

    // Check if the request was successful
    if res.is_err() {
        println!("Error: {}", res.err().unwrap());
        return;
    }

    // Get the response body in bytes
    let body = res.unwrap().bytes().unwrap();

    // Create a file in the output directory
    let mut file_ = std::fs::File::create(args.output.join("qrcode.png")).unwrap();

    // Save the body to file
    file_.write_all(&body).unwrap();

    
    let conf = Config {
        // set offset
        x: 4,
        y: 4,
        // set dimensions
        width: Some(80),
        height: Some(40),
        ..Default::default()
    };

    println!("QR code saved to {}", args.output.join("qrcode.png").display());

    if args.display {
        print_from_file(args.output.join("qrcode.png").to_str().unwrap(), &conf).expect("Image printing failed.");
    }
}
