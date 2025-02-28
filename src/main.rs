mod utils;

fn main() {
    //获取主机名
    match utils::hostname::get() {
        Ok(hostname) => println!("主机名: {}", hostname.to_string_lossy()),
        Err(e) => println!("获取主机名失败: {}", e),
    }
    //获取用户名
    match utils::user::get() {
        Ok(user) => println!("当前用户名: {}", user),
        Err(_) => println!("无法获取当前用户名"),
    }
    //------------------------------
    //获取主机信息 Host:
    match utils::host_model::get() {
        Ok(model) => println!("主机型号: {}", model),
        Err(e) => println!("获取主机型号失败: {}", e),
    }
    //获取Kernel信息 Kernel:
    match utils::kernel_version::get() {
        Ok(version) => println!("内核版本: {}", version),
        Err(e) => println!("获取内核版本失败: {}", e),
    }
    //获取系统运行时间
    match utils::uptime::get() {
        Ok(uptime) => println!(
            "系统运行时间: {} days, {} hours ,{} mins",
            uptime.num_days(),
            uptime.num_hours() % 24,
            uptime.num_minutes() % 60,
        ),
        Err(e) => println!("获取系统运行时间失败: {}", e),
    }

    // 获取系统信息
    match utils::os::get() {
        Ok(os_info) => {
            println!("OS:{}", os_info)
        }
        Err(e) => print!("获取 OS 失败{}", e),
    }
}
