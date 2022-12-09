use std::fs::{read_to_string, write};
use regex::Regex;


#[cfg(target_os = "macos")]
static HOSTS_PATH: &str = "/etc/hosts";

#[cfg(target_os = "linux")]
static HOSTS_PATH: &str = "/etc/hosts";

#[cfg(target_os = "windows")]
static HOSTS_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://raw.hellogithub.com/hosts").await?.text().await?;

    const START_FLAG: &str = "# GitHub Host";
    const END_FLAG: &str = "# GitHub Host END";

    let hosts = read_to_string(HOSTS_PATH).unwrap();

    if hosts.contains(START_FLAG) {
        let re = Regex::new(r"# GitHub Host[^:@]+# GitHub Host END").unwrap();

        let new_hosts = re.replace(&hosts, format!("{START_FLAG}{x}{resp}{x}{END_FLAG}", x = "\n")).to_string();

        write(HOSTS_PATH, new_hosts).unwrap();

    } else {
        let github_host = format!("{START_FLAG}{x}{resp}{x}{END_FLAG}", x = "\n");

        let new_hosts = format!("{hosts}\n\n{github_host}");

        write(HOSTS_PATH, new_hosts).unwrap();
    }
    
    Ok(())
}
