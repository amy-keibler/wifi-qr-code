use structopt::StructOpt;

use wifi_qr_code::QrCodeEcc;
use wifi_qr_code::{AuthenticationType, Visibility, WifiCredentials};

use std::fs::File;
use std::path::PathBuf;

#[derive(StructOpt)]
#[structopt(name = "qr_code_gen", about = "An example of using wifi-qr-code")]
struct Opt {
    #[structopt(short, long)]
    hidden: bool,

    #[structopt(short, long, default_value = "512")]
    size: usize,

    #[structopt(name = "SSID")]
    ssid: String,

    #[structopt(name = "FILE", parse(from_os_str))]
    png_file: PathBuf,
}

fn main() -> Result<(), std::io::Error> {
    let opt = Opt::from_args();
    let password =
        rpassword::read_password_from_tty(Some("Password: ")).expect("Failed to get password.");

    let visibility = if opt.hidden {
        Visibility::Hidden
    } else {
        Visibility::Visible
    };

    let wifi_credentials = WifiCredentials {
        ssid: opt.ssid,
        authentication_type: AuthenticationType::WPA(password),
        visibility,
    };

    let png_file = File::create(opt.png_file)?;
    wifi_qr_code::encode_as_png(&wifi_credentials, QrCodeEcc::Medium, opt.size, png_file)?;

    Ok(())
}
