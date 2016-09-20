extern crate time;

use std::io::prelude::*;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::Result;
use speedtest::cli::Speedtest;


pub fn fill(file_name: &str, connected: bool) -> Result<bool> {
  //2. Check if csv file exists
    if let Ok(file) = OpenOptions::new().write(true).append(true).open(file_name) {
        println!("File found");
        try!(update(file, connected));

       return  Ok(false);
    }

    try!(create(file_name));

    return Ok(true);
}

fn update(mut file: File, connected: bool) -> Result<()> {

     if connected {
        //5. If it is connected call the command `speed_test_csv` 
        println!("Checking the connections speed, this can take some time...");
        let speedtest = Speedtest::default();
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
    let speedtest = Speedtest{ add_header: true };
    let result = try!(speedtest.run());
    let s = &*result;
    
    println!("File not found, creating one...");
    let mut file = try!(File::create(file_name));
    let _ = file.write_all(s.as_bytes());

    Ok(())
}
