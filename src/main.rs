use std::process;
use windows::{
    core::*,
    Win32::UI::WindowsAndMessaging::*,
};
use github_hosts_updater::{fetch_data, update_hosts};

fn main() {
    let res = fetch_data();

    match res {
        Ok(str) => {
            if let Err(_) = update_hosts(str) {
                unsafe {
                    MessageBoxW(None, w!("hosts 文件读写失败！"), w!("提示"), MB_OK);
                }
                process::exit(1);
            };
        },
        Err(_) => {
            unsafe {
                MessageBoxW(None, w!("hosts 数据请求失败，请检查网络状况"), w!("提示"), MB_OK);
            }
        },
    }
}
