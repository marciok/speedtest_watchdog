extern crate speedtest_watchdog as speedtest;

use std::net::TcpStream;
use speedtest::csv::*;
use speedtest::g_drive::file::*;


const GOOGLE_DNS: &'static str = "8.8.8.8:53";
const FILE: &'static str = "speedtest.csv";

fn main() {

    //1. Verify is connected
    let connected = connect_with_ip(GOOGLE_DNS);
    println!("Connected: {}", connected);
    let created = writer::fill(FILE, connected).unwrap();

    //6. Parse the result into a Connection struct(TODO)
    //7. Otherwise create Connection with the default values(TODO)
    //8. Push connection into the file(TODO)

    //9. Create file hasn't already on Google Drive or just update 
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


fn connect_with_ip(ip: &str) -> bool {
    TcpStream::connect(ip).is_ok()
}
