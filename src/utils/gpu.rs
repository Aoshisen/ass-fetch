use std::process::Command;

pub fn get() -> String {
    let output = Command::new("system_profiler")
        .arg("SPDisplaysDataType")
        .output();
    match output {
        Ok(output) => {
            let output_str = String::from_utf8_lossy(&output.stdout);
            let mut gpu_info = String::new();
            // 查找包含 "Chipset Model:" 的行来获取 GPU 型号
            for line in output_str.lines() {
                if line.contains("Chipset Model:") {
                    if let Some(gpu_model) = line.split(":").nth(1) {
                        gpu_info = gpu_model.trim().to_string();
                        break;
                    }
                }
            }
            gpu_info
        }
        Err(_) => String::from("UNKNOWN"),
    }
}
