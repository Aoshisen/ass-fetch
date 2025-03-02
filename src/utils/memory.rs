use std::process::Command;

pub fn get() -> Result<String, std::io::Error> {
    // 获取总内存大小（字节）
    let total_memory = Command::new("sysctl")
        .arg("-n")
        .arg("hw.memsize")
        .output()?;
    
    let total_memory = String::from_utf8_lossy(&total_memory.stdout)
        .trim()
        .parse::<u64>()
        .unwrap_or(0);

    // 获取页面大小（字节）
    let page_size = Command::new("sysctl")
        .arg("-n")
        .arg("hw.pagesize")
        .output()?;

    let page_size = String::from_utf8_lossy(&page_size.stdout)
        .trim()
        .parse::<u64>()
        .unwrap_or(0);

    // 获取空闲页面数量
    let free_pages = Command::new("sysctl")
        .arg("-n")
        .arg("vm.page_free_count")
        .output()?;

    let free_pages = String::from_utf8_lossy(&free_pages.stdout)
        .trim()
        .parse::<u64>()
        .unwrap_or(0);

    // 计算空闲内存（字节）
    let free_memory = free_pages * page_size;

    // 转换为 GB 并保留两位小数
    let total_gb = total_memory as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_gb = (total_memory - free_memory) as f64 / 1024.0 / 1024.0 / 1024.0;

    Ok(format!("Memory: {:.2}GB / {:.2}GB", used_gb, total_gb))
}