use std::net::TcpStream;
use std::fs::File;
use std::process::Command;


const GOOGLE_DNS: &'static str = "8.8.8.8:53";
const FILE: &'static str = "speed_test.csv";

fn main() {

    //1. Verify is connected
    let connected = connect_with_ip(GOOGLE_DNS);
    println!("Connected: {}", connected);

    //2. Check if csv file exists
    if let Ok(file) = File::open(FILE) {
        println!("File found");
    } else {
        //3. If it doesn't create using string dumped from command `speedtest-csv --header` 
        println!("File not found, creating one...");
        let mut file = File::create(FILE).expect("Error while creating a file");
        
    }

    //4. If it does, check if is connected
    //5. If it is connected call the command `speed_test_csv` 
    //6. Parse the result into a Connection struct
    //7. Otherwise create Connection with the default values
    //8. Push connection into the file
    //9. Create file hasn't already on Google Drive or just update 

}

fn connect_with_ip(ip: &str) -> bool {
    TcpStream::connect(ip).is_ok()
}
