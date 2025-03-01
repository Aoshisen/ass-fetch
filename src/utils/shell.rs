use std::str;
use std::str::FromStr;
use std::{env, process::Command};

use regex::Regex;

pub fn get() -> Option<String> {
    let output = Command::new(env::var("SHELL").unwrap())
        .arg("--version")
        .output()
        .expect("Failed to execute command");
    let shell = str::from_utf8(&output.stdout).unwrap().to_string();
    if let Ok(shell_info) = extract_shell_info(&shell) {
        return Some(shell_info);
    }
    Some(shell)
}
pub fn extract_shell_info(shell_info: &str) -> Result<String, String> {
    // 匹配 zsh 5.9 (arm64-apple-darwin24.0) 中zsh 5.9
    let re = Regex::new(r"(.+)\(.+\)").map_err(|_| "Failed to compile regex".to_string())?;
    if let Some(captures) = re.captures(shell_info) {
        if let Some(sec_str) = captures.get(1) {
            return String::from_str(sec_str.as_str())
                .map_err(|_| "Failed to parse shell info".to_string());
        }
    }

    Err("Failed to find sec value in boot time".to_string())
}
