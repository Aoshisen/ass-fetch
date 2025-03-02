mod utils;

fn main() {
    let mut info_lines = Vec::new();

    // 获取主机名
    info_lines.push(format!("Hostname: {}", utils::hostname::get().unwrap_or_else(|e| e.to_string())));

    // 获取用户名
    info_lines.push(format!("Username: {}", utils::user::get().unwrap_or_else(|_| String::from("无法获取当前用户名"))));

    // 获取系统信息
    info_lines.push(format!("OS: {}", utils::os::get().unwrap_or_else(|e| format!("获取 OS 失败{}", e))));

    // 获取主机信息
    info_lines.push(format!("Host: {}", utils::host_model::get().unwrap_or_else(|e| format!("获取主机型号失败: {}", e))));

    // 获取 Kernel 信息
    info_lines.push(format!("Kernel: {}", utils::kernel_version::get().unwrap_or_else(|e| format!("获取内核版本失败: {}", e))));

    // 获取系统运行时间
    info_lines.push(format!("Uptime: {}", utils::uptime::get().map_or_else(
        |e| format!("获取系统运行时间失败: {}", e),
        |uptime| format!("{} days, {} hours, {} mins",
            uptime.num_days(),
            uptime.num_hours() % 24,
            uptime.num_minutes() % 60
        )
    )));

    // 获取包信息
    info_lines.push(format!("Packages: {}", utils::packages::get().unwrap_or_else(|e| format!("获取系统创建包个数失败: {}", e))));

    // 获取 Shell 信息
    if let Some(shell) = utils::shell::get() {
        info_lines.push(format!("Shell: {}", shell));
    }

    // 获取分辨率
    if let Ok(resolution) = utils::resolution::get() {
        info_lines.push(format!("Resolution: {}", resolution));
    }

    info_lines.push(format!("DE: {}", utils::de::get()));
    info_lines.push(format!("WM: {}", utils::wm::get()));
    info_lines.push(format!("Theme: {}", utils::wm::get_theme()));
    info_lines.push(format!("Terminal: {}", utils::terminal::get()));
    info_lines.push(format!("Terminal Font: {}", utils::terminal::font()));

    // 获取 CPU 信息
    info_lines.push(format!("CPU: {}", utils::cpu::get().unwrap_or_else(|e| format!("获取 CPU 信息失败: {}", e))));

    // 获取 GPU 信息
    info_lines.push(format!("GPU: {}", utils::gpu::get().unwrap_or_else(|e| format!("获取 GPU 信息失败: {}", e))));

    // 获取内存信息
    info_lines.push(format!("Memory: {}", utils::memory::get().unwrap_or_else(|e| format!("获取内存信息失败: {}", e))));

    // 获取 ASCII art
    let ascii_art = utils::distro::get_distro_ascii();
    let ascii_lines: Vec<&str> = ascii_art.split('\n').collect();

    // 计算最大行数
    let max_lines = ascii_lines.len().max(info_lines.len());

    // 输出左右布局
    for i in 0..max_lines {
        let ascii_line = if i < ascii_lines.len() {
            ascii_lines[i]
        } else {
            ""
        };

        let info_line = if i < info_lines.len() {
            &info_lines[i]
        } else {
            ""
        };

        println!("{:<42}{}", ascii_line, info_line);
    }
}
