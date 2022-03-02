use std::io::Read;
use std::io::Write;
use clap::Parser;

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
    let mut res = ureq::get(format!("https://api.dhravya.me/qrcode?query={}", args.data).as_str())
        .call()
        .unwrap()
        .into_reader();
    
    // Get the image data
    let mut data = Vec::new();
    res.read_to_end(&mut data).unwrap();

    // Write the image to the file
    let mut file = std::fs::File::create(args.output.join("qrcode.png")).unwrap();
    file.write_all(&data).unwrap();

    println!("QR code saved to {}", args.output.join("qrcode.png").display());

}
