use std::process::Command;

pub fn get() -> String {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("hw.model") // 获取主机型号
        .output();

    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => String::from("UNKNOWN")
    }
}
