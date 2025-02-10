use colored::*;

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
}