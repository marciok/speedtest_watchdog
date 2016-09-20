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

        let output = try!(Command::new("utils/speedtest-cli-extras/bin/speedtest-csv").args(&args).output());
        let output_string = String::from_utf8(output.stdout).unwrap();

        Ok(output_string)
    }
}
