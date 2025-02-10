use colored::*;

use colored::Colorize;
use log::{debug, error, info, trace, warn};
use env_logger::Builder;
use std::io::Write;
// 配置日志记录器
fn setup_logger() {
    let mut builder = Builder::new();
    // 自定义日志输出格式
    builder.format(|buf, record| {
        let level_str = match record.level() {
            log::Level::Error => record.level().to_string().red(),
            log::Level::Warn => record.level().to_string().yellow(),
            log::Level::Info => record.level().to_string().green(),
            log::Level::Debug => record.level().to_string().blue(),
            log::Level::Trace => record.level().to_string().purple(),
        };
        // 获取日志消息
        let message = record.args().to_string();
        // 根据日志级别为消息添加颜色
        let colored_message = match record.level() {
            log::Level::Error => message.red(),
            log::Level::Warn => message.yellow(),
            log::Level::Info => message.green(),
            log::Level::Debug => message.blue(),
            log::Level::Trace => message.purple(),
        };
        // 写入格式化后的日志行
        writeln!(buf, "[{}] - {}", level_str, colored_message)
    });
    // 解析默认环境变量并初始化日志记录器
    builder.parse_default_env().init();
}
fn main() {
    // INFO: Hello, world!，其中 INFO: 为绿色，后续内容为白色
    println!("{}: {}", "INFO".green(), "Hello, world!".white());

    // WARNING: I'm a teapot!，颜色为黄色，加粗，并为 WARNING 添加下划线
    println!("{}: {}", "WARNING".yellow().bold().underline(), "I'm a teapot!".yellow().bold());

    // ERROR: KERNEL PANIC!!!，颜色为红色，加粗，并尝试让这一行在控制行窗口居中
    let error_msg = format!("{}: {}", "ERROR".red().bold(), "KERNEL PANIC!!!".red().bold());
    let padding = (80 - error_msg.len()) / 2; // 假设终端宽度为80
    println!("{:^80}", error_msg); // 使用格式化字符串居中

    // 其他效果和内容
    println!("{}", "This is a cyan text on a black background".cyan().on_black());
    println!("{}", "This is a magenta text with a blue background".magenta().on_blue());
    println!("{}", "This is a bold red text".red().bold());
    println!("{}", "This is an italic green text".green().italic());

    setup_logger();
    trace!("This is a trace log.");
    debug!("This is a debug log.");
    info!("This is an info log.");
    warn!("This is a warning log.");
    error!("This is an error log.");
}