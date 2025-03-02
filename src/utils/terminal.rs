use std::env;
use std::process::Command;

pub fn get() -> String {
    // 首先尝试从 TERM_PROGRAM 环境变量获取
    if let Ok(term) = env::var("TERM_PROGRAM") {
        return term;
    }

    // 如果 TERM_PROGRAM 不存在，尝试从 TERM 环境变量获取
    if let Ok(term) = env::var("TERM") {
        return term;
    }

    // 如果都获取不到，返回默认值
    String::from("unknown")
}

pub fn font() -> String {
    // 对于 macOS，我们可以尝试使用 osascript 获取终端字体信息
    let output = Command::new("osascript")
        .arg("-e")
        .arg("tell application \"Terminal\" to font name of window 1")
        .output();

    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => {
            // 如果获取失败，尝试从 TERM_FONT 环境变量获取
            env::var("TERM_FONT").unwrap_or_else(|_| String::from("unknown"))
        }
    }
}
