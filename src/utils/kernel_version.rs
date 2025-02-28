use std::process::Command;
pub fn get() -> Result<String, std::io::Error> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("kern.osrelease") // 获取内核版本
        .output()?;

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(version)
}
