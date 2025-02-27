use hostname;
use regex::Regex;
use std::env;
use std::process::Command;

fn main() {
    //获取主机名
    match hostname::get() {
        Ok(hostname) => println!("主机名: {}", hostname.to_string_lossy()),
        Err(e) => println!("获取主机名失败: {}", e),
    }
    //获取用户名
    match env::var("USER") {
        Ok(user) => println!("当前用户名: {}", user),
        Err(_) => match env::var("LOGNAME") {
            Ok(user) => println!("当前用户名: {}", user),
            Err(_) => println!("无法获取当前用户名"),
        },
    }
    //------------------------------
    //获取主机信息 Host:
    match get_host_model() {
        Ok(model) => println!("主机型号: {}", model),
        Err(e) => println!("获取主机型号失败: {}", e),
    }
    //获取Kernel信息 Kernel:
    match get_kernel_version() {
        Ok(version) => println!("内核版本: {}", version),
        Err(e) => println!("获取内核版本失败: {}", e),
    }
    //获取系统运行时间
    match get_uptime() {
        Ok(uptime) => println!("系统运行时间: {}", uptime),
        Err(e) => println!("获取系统运行时间失败: {}", e),
    }
}

fn get_host_model() -> Result<String, std::io::Error> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("hw.model") // 获取主机型号
        .output()?;

    let model = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(model)
}

fn get_kernel_version() -> Result<String, std::io::Error> {
    let output = Command::new("sysctl")
        .arg("-n")
        .arg("kern.osrelease") // 获取内核版本
        .output()?;

    let version = String::from_utf8_lossy(&output.stdout).trim().to_string();
    Ok(version)
}

fn get_uptime() -> Result<String, std::io::Error> {
    let output = Command::new("uptime").output()?;

    let uptime = String::from_utf8_lossy(&output.stdout).trim().to_string();

    match parse_uptime(&uptime) {
        Ok(formatted_uptime) => Ok(formatted_uptime),
        Err(_) => Err(std::io::Error::new(
            std::io::ErrorKind::Other,
            "时间解析错误",
        )),
    }
}

fn parse_uptime(uptime: &str) -> Result<String, String> {
    // 定义正则表达式以提取天数和时间
    let re =
        Regex::new(r"up (\d+) days?,\s*(\d+):(\d+),\s*\d+ users?").map_err(|e| e.to_string())?;

    if let Some(caps) = re.captures(uptime) {
        // 提取天数、小时和分钟
        let days = &caps[1];
        let hours = &caps[2].parse::<u32>().unwrap_or(0); // 转换为数字并去除前导零
        let minutes = &caps[3].parse::<u32>().unwrap_or(0); // 转换为数字并去除前导零

        // 格式化并返回字符串
        Ok(format!(
            "系统运行时间: {}天, {}小时 {}分钟",
            days, hours, minutes
        ))
    } else {
        Err("无法解析uptime信息".to_string())
    }
}
