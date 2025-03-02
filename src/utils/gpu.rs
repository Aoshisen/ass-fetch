use std::process::Command;

pub fn get() -> Result<String, std::io::Error> {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output()?;

    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // 查找包含 "Chipset Model:" 的行来获取 GPU 型号
    for line in output_str.lines() {
        if line.contains("Chipset Model:") {
            if let Some(gpu_model) = line.split(":").nth(1) {
                return Ok(gpu_model.trim().to_string());
            }
        }
    }

    Ok(String::from("Unknown GPU"))
}