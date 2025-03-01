use std::process::Command;
use std::str::{self, FromStr};

use regex::Regex;

pub fn get() -> Result<String, String> {
    let output = Command::new("screenresolution")
        .arg("get")
        .output()
        .unwrap();
    let screen_resolution = str::from_utf8(&output.stderr).unwrap().to_string();
    if let Ok(resolution) = extract_screen_resolution(&screen_resolution) {
        return Ok(resolution);
    }
    Err("fail to extract_screen_resolution".to_string())
}

pub fn extract_screen_resolution(screen_info: &str) -> Result<String, String> {
    let re = Regex::new(r"Display.+:(.+)").map_err(|_| "FAIL".to_string())?;
    if let Some(captures) = re.captures(screen_info) {
        if let Some(sec_str) = captures.get(1) {
            return String::from_str(sec_str.as_str())
                .map_err(|_| "Failed to parse shell info".to_string());
        }
    }

    Err("fail to extract_screen_resolution".to_string())
}
