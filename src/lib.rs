/*
 * @Author: thelostword
 * @Date: 2022-12-19 14:53:27
 * @LastEditors: thelostword
 * @LastEditTime: 2022-12-19 14:59:07
 * @FilePath: \github_hosts_updater\src\lib.rs
 */
use std::{
    error::Error,
    result::Result,
    fs::{read_to_string, write}
};
use regex::Regex;
use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
};


#[cfg(target_os = "macos")]
static HOSTS_PATH: &str = "/etc/hosts";

#[cfg(target_os = "linux")]
static HOSTS_PATH: &str = "/etc/hosts";

#[cfg(target_os = "windows")]
static HOSTS_PATH: &str = "C:\\Windows\\System32\\drivers\\etc\\hosts";


#[tokio::main]
pub async fn fetch_data() -> Result<String, Box<dyn Error>> {
    let resp = reqwest::get("https://raw.hellogithub.com/hosts").await?.text().await?;
    Ok(resp)
}

pub fn update_hosts(resp: String) -> Result<(), Box<dyn Error>> {
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

    unsafe {
        MessageBoxW(None, w!("hosts 更新成功！"), w!("提示"), MB_OK);
    }

    Ok(())
}
