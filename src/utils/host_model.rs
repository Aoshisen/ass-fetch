use std::process::Command;
pub fn get() -> Result<String, std::io::Error> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("hw.model") // 获取主机型号
        .output()?;

    let model = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(model)
}