use std::process::Command;

pub fn get() -> String {
    // 获取总内存大小（字节）
    let total_memory = match Command::new("sysctl").arg("-n").arg("hw.memsize").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u64>()
            .unwrap_or(0),
        Err(_) => return String::from("UNKNOWN"),
    };

    // 获取页面大小（字节）
    let page_size = match Command::new("sysctl").arg("-n").arg("hw.pagesize").output() {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u64>()
            .unwrap_or(0),
        Err(_) => return String::from("UNKNOWN"),
    };

    // 获取空闲页面数量
    let free_pages = match Command::new("sysctl")
        .arg("-n")
        .arg("vm.page_free_count")
        .output()
    {
        Ok(output) => String::from_utf8_lossy(&output.stdout)
            .trim()
            .parse::<u64>()
            .unwrap_or(0),
        Err(_) => return String::from("UNKNOWN"),
    };

    // 计算空闲内存（字节）
    let free_memory = free_pages * page_size;

    // 转换为 GB 并保留两位小数
    let total_gb = total_memory as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_gb = (total_memory - free_memory) as f64 / 1024.0 / 1024.0 / 1024.0;

    format!("Memory: {:.2}GB / {:.2}GB", used_gb, total_gb)
}
