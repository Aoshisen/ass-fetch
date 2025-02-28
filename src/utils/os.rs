use std::io;
use std::process::Command;

fn parse_output(output: io::Result<std::process::Output>, error_message: &str) -> String {
    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => panic!("parse_output with error{}", error_message),
    }
}

pub fn get_os_name() -> String {
    let output = Command::new("uname").output();
    parse_output(output, "Failed to get OS name")
}

pub fn get_os_version() -> String {
    let output = Command::new("sw_vers")
        .arg("-productVersion") // 获取macOS版本
        .output();

    parse_output(output, "Failed to get OS version")
}

pub fn get_build_version() -> String {
    let output = Command::new("sw_vers")
        .arg("-buildVersion") // 获取内核版本
        .output();

    parse_output(output, "Failed to get kernel version")
}

pub fn get_architecture() -> String {
    let output = Command::new("uname")
        .arg("-m") // 获取架构
        .output();

    parse_output(output, "Failed to get architecture")
}

pub fn get() -> Result<String, io::Error> {
    let os_name = get_os_name();
    let os_version = get_os_version();
    let kernel_version = get_build_version();
    let architecture = get_architecture();
    Ok(format!(
        "{} {} {} {}",
        os_name, os_version, kernel_version, architecture
    ))
}
