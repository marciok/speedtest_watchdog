# Speedtest_Watchdog
Check your internet performance and upload the results to Google Drive.

Do you think your internet is intermitent, and slower than what was agreed on your contract?
If yes this tool, can be helpful. 
**Speedtest_Watchdog** will check if you have access to the internet, verifies the speed, and upload eveything into a csv on Google Drive.

Inspired by: [Use Raspberry Pi to Measure Broadband Speeds to Hold Your ISP Accountable](http://makezine.com/projects/send-ticket-isp-when-your-internet-drops/)

**Attention**: This is my first [Rust](https://www.rust-lang.org) project. It's just a toy :)

## Install:
1. Clone the project
2. Install [speedtest-cli](https://github.com/sivel/speedtest-cli) and [speedtest-cli-extras](https://github.com/HenrikBengtsson/speedtest-cli-extras)
2. Build, `cargo build --release`
3. Set the environment variables needed to authenticate with Google Drive:
```
export G_DRIVE_API_ID="loremipsum"
export G_DRIVE_API_SECRET="loremipsum"
export G_DRIVE_API_PROJECT="loremipsum"
export G_DRIVE_API_EMAIL="loremipsum@loremipsum.com"
```
4. Make sure `speedtest-csv` is reachable on your `$PATH`
5. run `./target/release/speedtest_watchdog`
6. Proofit!




