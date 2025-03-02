use std::process::{Command, Stdio};

pub fn get() -> String {
    let wms = vec![
        String::from("Quartz Compositor"),
        String::from("yabai"),
        String::from("Rectangle"),
        String::from("Spectacle"),
        String::from("Amethyst"),
        String::from("Kwm"),
    ];

    for wm in wms {
        // 执行 ps -e 命令
        let ps_cmd = Command::new("ps")
            .arg("-e")
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();

        // 使用 rg 命令过滤进程
        let rg_output = Command::new("rg")
            .arg("-o")
            .arg(&wm)
            .stdin(ps_cmd.stdout.unwrap())
            .output();

        match rg_output {
            Ok(output) if output.status.success() => {
                let result = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !result.is_empty() {
                    return result;
                }
            }
            _ => continue,
        }
    }

    String::from("None")
}

pub fn get_theme() -> String {
    // 获取主题模式 (Light/Dark)
    let theme = Command::new("defaults")
        .args(["read", "-g", "AppleInterfaceStyle"])
        .output()
        .map(|output| {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout).trim().to_string()
            } else {
                String::from("Light")
            }
        })
        .unwrap_or_else(|_| String::from("Light"));

    // 获取强调色
    let accent_color = Command::new("defaults")
        .args(["read", "-g", "AppleAccentColor"])
        .output()
        .map(|output| {
            if output.status.success() {
                String::from_utf8_lossy(&output.stdout)
                    .trim()
                    .parse::<i32>()
                    .unwrap_or(-1)
            } else {
                -1
            }
        })
        .unwrap_or(-1);

    let color = match accent_color {
        -1 => "Graphite",
        0 => "Red",
        1 => "Orange",
        2 => "Yellow",
        3 => "Green",
        5 => "Purple",
        6 => "Pink",
        _ => "Blue",
    };

    format!("{} ({})", color, theme)
}
