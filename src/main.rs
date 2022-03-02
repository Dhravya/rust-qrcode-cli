use clap::Parser;
use std::io::Read;
use std::io::Write;

// Simple CLI to make QR codes
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    // The information contained by the QR code
    #[clap(short = 'd', long = "data")]
    data: String,

    // Output folder
    #[clap(short = 'o', long = "output", default_value = ".")]
    #[clap(parse(from_os_str))]
    output: std::path::PathBuf,

    // Drawer to use, should be a value between 1 and 6 (included)
    #[clap(long = "drawer", default_value = "1")]
    drawer: u8,

    // Mask to use, should be a number between 1 and 5 (included)
    #[clap(long = "mask", short = 'm', default_value = "1")]
    mask: u8,

    // A valid color name or color code
    #[clap(long = "foreground", short = 'f', default_value = "black")]
    foreground: String,

    // A valid color name or color code
    #[clap(long = "background", short = 'b', default_value = "white")]
    background: String,
}

fn main() {
    let args = Cli::parse();

    // Check if foreground and background in colors
    // if !colors.contains(&args.foreground.as_str()) {
    //     println!("Invalid foreground color");
    //     return;
    // }

    assert!(
        args.data.len() <= 255,
        "Data must be less than 255 characters"
    );
    assert!(args.mask <= 5, "Mask must be less than 6");
    assert!(args.drawer <= 6, "Drawer number must be between 1 and 6");

    let mut url = format!(
            "https://api.dhravya.me/qrcode?query={}",
            args.data
        );

    if args.mask != 1 {
        url.push_str(&format!("&mask={}", args.mask));
        url.push_str(&format!("&drawer={}", args.drawer));
        println!("Warning: If mask and drawer are provided, there will be no FG and BG");
    }
    else{
        url.push_str(&format!("&fg={}&bg={}", args.foreground, args.background));
    }
    // Using blocking request
    let mut res = ureq::get(
        url.as_str()
    )
    .call()
    .unwrap()
    .into_reader();

    // Get the image data
    let mut data = Vec::new();
    res.read_to_end(&mut data).unwrap();
    
    // Check if data is error
    if data.starts_with(b"{\"success\":0,") {
        println!("{}", String::from_utf8(data).unwrap());
        return;
    }

    // Write the image to the file
    let mut file = std::fs::File::create(args.output.join("qrcode.png")).unwrap();
    file.write_all(&data).unwrap();

    println!(
        "QR code saved to {}",
        args.output.join("qrcode.png").display()
    );
}
