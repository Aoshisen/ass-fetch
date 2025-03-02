use std::process::Command;
pub fn get() -> String {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("kern.osrelease") // 获取内核版本
        .output();

    match output {
        Ok(output) => String::from_utf8_lossy(&output.stdout).trim().to_string(),
        Err(_) => String::from("UNKNOWN"),
    }
}
