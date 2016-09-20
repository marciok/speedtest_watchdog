extern crate speedtest_watchdog as speedtest;

use std::net::TcpStream;
use speedtest::csv::*;
use speedtest::g_drive::file::*;

const GOOGLE_DNS: &'static str = "8.8.8.8:53";
const FILE: &'static str = "speedtest.csv";

fn main() {

    let connected = TcpStream::connect(GOOGLE_DNS).is_ok();
    println!("Connected: {}", connected);

    let created = writer::fill(FILE, connected).expect("Error while filling the csv");

    if connected {
        println!("Uploading file");
        let config = UploaderConfig{ 
            id: env!("G_DRIVE_API_ID").to_string(),
            secret: env!("G_DRIVE_API_SECRET").to_string(),
            project: env!("G_DRIVE_API_PROJECT").to_string(),
            email: env!("G_DRIVE_API_EMAIL").to_string(),
        };

        let uploader = Uploader::new(config);
        println!("Finished: {:?}", uploader.upload(FILE, created));
    }
}
