use std::process::Command;

pub fn get() -> Result<String, std::io::Error> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("machdep.cpu.brand_string") // 获取 CPU 型号
        .output()?;

    let cpu_model = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(cpu_model)
}