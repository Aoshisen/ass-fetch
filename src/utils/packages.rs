use std::io;
use std::process::{Command, Stdio};

fn parse_output(output: io::Result<std::process::Output>, error_message: &str) -> String {
    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        }
        _ => panic!("parse_output with error{}", error_message),
    }
}

fn get_brew_list_num() -> String {
    // 执行 brew list
    let brew_list = Command::new("brew")
        .arg("list")
        .arg("--formula")
        .stdout(Stdio::piped()) // 将标准输出重定向到管道
        .spawn()
        .unwrap(); // 启动子进程

    // 使用 wc -l 来计算行数
    let wc_l = Command::new("wc")
        .arg("-l")
        .stdin(brew_list.stdout.unwrap()) // 将 brew list 的输出作为 wc -l 的输入
        .output(); // 获取 wc -l 命令的输出

    parse_output(wc_l, "获取brew list 数量失败")
}

fn get_port_list_num() -> String {
    let port_list = Command::new("port")
        .arg("installed")
        .stdout(Stdio::piped())
        .spawn()
        .unwrap();

    let wc_l = Command::new("wc")
        .arg("-l")
        .stdin(port_list.stdout.unwrap())
        .output();

    // 将输出转换为字符串并返回
    parse_output(wc_l, "获取port信息失败")
}

pub fn get() -> Result<String, io::Error> {
    let brew_list = get_brew_list_num();
    let port_list = get_port_list_num();
    Ok(format!("{} (brew), {} (port)", brew_list, port_list))
}
