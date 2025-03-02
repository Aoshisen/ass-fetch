mod utils;
fn main() {
    let info_lines = collect_system_info();
    let ascii_art = utils::distro::get_distro_ascii();
    display_layout(info_lines, ascii_art);
}

fn collect_system_info() -> Vec<String> {
    let info_configs: [(_, Box<dyn Fn() -> String>); 17] = [
        ("Hostname", Box::new(|| utils::hostname::get())),
        ("Username", Box::new(|| utils::user::get())),
        ("OS", Box::new(|| utils::os::get())),
        ("Host", Box::new(|| utils::host_model::get())),
        ("Kernel", Box::new(|| utils::kernel_version::get())),
        ("Uptime", Box::new(|| utils::uptime::get())),
        ("Packages", Box::new(|| utils::packages::get())),
        ("Shell", Box::new(|| utils::shell::get())),
        ("Resolution", Box::new(|| utils::resolution::get())),
        ("DE", Box::new(|| utils::de::get())),
        ("WM", Box::new(|| utils::wm::get())),
        ("Theme", Box::new(|| utils::wm::get_theme())),
        ("Terminal", Box::new(|| utils::terminal::get())),
        ("Terminal Font", Box::new(|| utils::terminal::font())),
        ("CPU", Box::new(|| utils::cpu::get())),
        ("GPU", Box::new(|| utils::gpu::get())),
        ("Memory", Box::new(|| utils::memory::get())),
    ];

    info_configs
        .iter()
        .map(|(label, getter)| format!("{}: {}", label, getter()))
        .collect()
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
