mod utils;
fn main() {
    let info_lines = collect_system_info();
    let ascii_art = utils::distro::get_distro_ascii();
    display_layout(info_lines, ascii_art);
}

fn collect_system_info() -> Vec<String> {
    let mut info_lines = Vec::new();
    // 获取主机名
    info_lines.push(format!("Hostname: {}", utils::hostname::get()));
    // 获取用户名
    info_lines.push(format!("Username: {}", utils::user::get()));
    // 获取系统信息
    info_lines.push(format!("OS: {}", utils::os::get()));
    // 获取主机信息
    info_lines.push(format!("Host: {}", utils::host_model::get()));
    // 获取 Kernel 信息
    info_lines.push(format!("Kernel: {}", utils::kernel_version::get()));
    // 获取系统运行时间
    info_lines.push(format!("Uptime: {}", utils::uptime::get()));
    // 获取包信息
    info_lines.push(format!("Packages: {}", utils::packages::get()));
    // 获取 Shell 信息
    info_lines.push(format!("Shell: {}", utils::shell::get()));
    // 获取分辨率
    info_lines.push(format!("Resolution: {}", utils::resolution::get()));
    //获取DE:
    info_lines.push(format!("DE: {}", utils::de::get()));
    //获取WM:
    info_lines.push(format!("WM: {}", utils::wm::get()));
    //获取Theme:
    info_lines.push(format!("Theme: {}", utils::wm::get_theme()));
    //获取终端:
    info_lines.push(format!("Terminal: {}", utils::terminal::get()));
    //获取终端字体
    info_lines.push(format!("Terminal Font: {}", utils::terminal::font()));
    // 获取 CPU 信息
    info_lines.push(format!("CPU: {}", utils::cpu::get()));
    // 获取 GPU 信息
    info_lines.push(format!("GPU: {}", utils::gpu::get()));
    // 获取内存信息
    info_lines.push(format!("Memory: {}", utils::memory::get()));

    info_lines
}

fn display_layout(info_lines: Vec<String>, ascii_art: String) {
    let ascii_lines: Vec<&str> = ascii_art.split('\n').collect();
    let max_lines = ascii_lines.len().max(info_lines.len());

    // 计算 ASCII 艺术的最大宽度
    let max_ascii_width = ascii_lines.iter().map(|line| line.len()).max().unwrap_or(0);

    // 设置基础间距为 10 个字符
    let base_padding = 10;
    // 计算总宽度（ASCII宽度 + 基础间距）
    let total_width = max_ascii_width + base_padding;

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

        println!("{:<width$}{}", ascii_line, info_line, width = total_width);
    }
}
