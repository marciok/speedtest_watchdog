use std::process::Command;
use std::io;

#[derive(Default)]
pub struct Speedtest {
    pub add_header: bool,
}

impl Speedtest {

    pub fn run(&self) -> io::Result<String> {
        let mut args = vec!["--sep", "','"];

        if self.add_header {
            args.push("--header");
        }

        println!("Running command...");
        let output = try!(Command::new("speedtest-csv")
                          .args(&args)
                          .output());
        println!("speedtest-csv exited with status: {:?}", output.status.code());
        assert!(output.status.success());
        let output_string = String::from_utf8(output.stdout).expect("Didn't get any result from speedtest-csv, make sure it's installed");

        Ok(output_string)
    }
}
