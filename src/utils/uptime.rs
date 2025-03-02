use chrono::{DateTime, TimeDelta, Utc};
use regex::Regex;
use std::process::Command;
use std::str::FromStr;

pub fn get() -> String {
    // 执行 `uptime` 命令获取系统的运行时间
    let output = match Command::new("sysctl")
        .arg("-n")
        .arg("kern.boottime")
        .output()
    {
        Ok(output) => output,
        Err(_) => return String::from("UNKNOWN"),
    };

    if !output.status.success() {
        return String::from("UNKNOWN");
    }

    // 获取命令的输出，并处理为字符串
    let boot_time_str = String::from_utf8_lossy(&output.stdout).trim().to_string();

    match extract_sec_from_boot_time(&boot_time_str) {
        Ok(formatted_uptime) => {
            let uptime = calculate_uptime(formatted_uptime);
            format_uptime(uptime)
        }
        Err(_) => String::from("UNKNOWN"),
    }
}

fn format_uptime(uptime: TimeDelta) -> String {
    format!(
        "{} days, {} hours, {} mins",
        uptime.num_days(),
        uptime.num_hours() % 24,
        uptime.num_minutes() % 60
    )
}

fn extract_sec_from_boot_time(boot_time_str: &str) -> Result<i64, String> {
    let re = Regex::new(r"sec\s*=\s*(\d+)").map_err(|_| "Failed to compile regex".to_string())?;
    if let Some(captures) = re.captures(boot_time_str) {
        if let Some(sec_str) = captures.get(1) {
            return i64::from_str(sec_str.as_str()).map_err(|_| "Failed to parse sec".to_string());
        }
    }
    Err("Failed to find sec value in boot time".to_string())
}

fn calculate_uptime(boot_time: i64) -> TimeDelta {
    let start_time = Utc::now();
    let end_time = DateTime::from_timestamp(boot_time, 0).unwrap();
    let duration = start_time - end_time;
    duration
}
