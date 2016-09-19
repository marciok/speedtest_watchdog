extern crate csv;
extern crate time;
extern crate hyper;
extern crate yup_oauth2 as oauth2;
extern crate google_drive3 as drive3;

use std::net::TcpStream;
use std::fs::File;
use std::fs::OpenOptions;
use std::process::Command;
use std::io::prelude::*;
use std::io::Result;
use oauth2::{Authenticator, DefaultAuthenticatorDelegate, ApplicationSecret, DiskTokenStorage};
use drive3::Drive;

const GOOGLE_DNS: &'static str = "8.8.8.8:53";
const FILE: &'static str = "speedtest.csv";
const JSON_SECRET: &'static str = "authfile";
const MIME_TYPE: &'static str = "application/vnd.google-apps.spreadsheet";
const AUTH_URI: &'static str = "https://accounts.google.com/o/oauth2/auth";
const TOKEN_URI: &'static str = "https://accounts.google.com/o/oauth2/token";

#[derive(Default)]
struct SpeedtestCLI {
    add_header: bool,
}

impl SpeedtestCLI {

    fn run(&self) -> Result<String> {
        let mut args = vec!["--sep", "','"];

        if self.add_header {
            args.push("--header");
        }

        let output = try!(Command::new("utils/speedtest-cli-extras/bin/speedtest-csv").args(&args).output());
        let output_string = String::from_utf8(output.stdout).unwrap();

        Ok(output_string)
    }
}

fn main() {

    //1. Verify is connected
    let connected = connect_with_ip(GOOGLE_DNS);
    println!("Connected: {}", connected);
    let created = create_or_update(FILE, connected).unwrap();

    //6. Parse the result into a Connection struct(TODO)
    //7. Otherwise create Connection with the default values(TODO)
    //8. Push connection into the file(TODO)

    //9. Create file hasn't already on Google Drive or just update 
    if connected {
        println!("Uploading file");
        println!("Finished: {:?}", upload(FILE, created));
    }
}

fn upload(file_name: &str, created: bool) -> std::result::Result<(), drive3::Error> {
    let mut file_id: Option<String> = None;

    let secret: ApplicationSecret = ApplicationSecret { 
        client_id: env!("G_DRIVE_API_ID").to_string(),
        client_secret:env!("G_DRIVE_API_SECRET").to_string(),
        project_id: Some(env!("G_DRIVE_API_PROJECT").to_string()),
        client_email: Some(env!("G_DRIVE_API_EMAIL").to_string()),
        auth_uri:AUTH_URI.to_string(),
        token_uri:TOKEN_URI.to_string(),
        ..ApplicationSecret::default()
    };

    let auth = Authenticator::new(&secret, DefaultAuthenticatorDelegate,
                                  hyper::Client::new(),
                                  DiskTokenStorage::new(&JSON_SECRET.to_string()).unwrap(), 
                                  Some(oauth2::FlowType::InstalledInteractive));

    let hub = Drive::new(hyper::Client::new(), auth);
    let req = drive3::File { 
        mime_type: Some(MIME_TYPE.to_string()),
        is_app_authorized: Some(true),
        name: Some(file_name.to_string()),
        ..drive3::File::default() 
    };

    if created {
        println!("Creating file on Google Drive");
        try!(hub.files()
                .create(req)
                .upload_resumable(
                    File::open(file_name).unwrap(),
                    MIME_TYPE.parse().unwrap()
                ));

        return Ok(());
    }


    if let None = file_id {
        let result = try!(hub.files()
                          .list()
                          .q(&*format!("name='{}'", file_name))
                          .doit());

        file_id = Some(extract_id(&result.1));
    }
    println!("Updating file");

    try!(hub.files()
            .update(req, &*file_id.expect("File id is empty"))
            .upload_resumable(
                File::open(FILE).unwrap(),
                MIME_TYPE.parse().unwrap()
            ));

    Ok(())
}

fn extract_id(file_list: &drive3::FileList) -> String {
    let f = file_list.files.clone();
    let files_unwraped = f.unwrap();
    let first_file = files_unwraped.first();
    let first_file_unwraped = first_file.unwrap();
    let final_file_id = first_file_unwraped.id.clone();
    let id_unwraped = final_file_id.unwrap().clone();
    
    return id_unwraped;
}

fn update(mut file: File, connected: bool) -> Result<()> {

     if connected {
        //5. If it is connected call the command `speed_test_csv` 
        println!("Checking the connections speed, this can take some time...");
        let speedtest = SpeedtestCLI::default();
        let result = try!(speedtest.run());
        println!("Test finished");
        let s = &*result;
        let _ = file.write_all(s.as_bytes());

        //4. If it does, check if is connected
    } else {
        let time = time::strftime("%Y-%m-%d %H:%M:%S", &time::now()).unwrap();
        let format = format!("{},,,,,,,,,\n", time);
        let _ = file.write_all(format.as_bytes());
    };
    Ok(())
}

//3. Create using string dumped from command `speedtest-csv --header` 
fn create(file_name: &str) -> Result<()> {
    let speedtest = SpeedtestCLI{ add_header: true };
    let result = try!(speedtest.run());
    let s = &*result;
    
    println!("File not found, creating one...");
    let mut file = try!(File::create(file_name));
    let _ = file.write_all(s.as_bytes());

    Ok(())
}

fn create_or_update(file_name: &str, connected: bool) -> Result<bool> {
  //2. Check if csv file exists
    if let Ok(file) = OpenOptions::new().write(true).append(true).open(file_name) {
        println!("File found");
        try!(update(file, connected));

       return  Ok(false);
    }

    try!(create(file_name));

    return Ok(true);
}

fn connect_with_ip(ip: &str) -> bool {
    TcpStream::connect(ip).is_ok()
}
