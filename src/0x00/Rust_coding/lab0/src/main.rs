use std::{fs::File, io::{self, Read}, thread, time::Duration};

/// 倒计时函数
fn count_down(seconds: u64) {
    for i in (1..=seconds).rev() {
        println!("{} seconds remaining...", i);
        thread::sleep(Duration::from_secs(1));
    }
    println!("Countdown finished!");
}

/// 读取并打印文件内容，若文件不存在则 panic
fn read_and_print(file_path: &str) -> io::Result<()> {
    let mut file = File::open(file_path).expect("File not found!");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    println!("{}", contents);
    Ok(())
}

/// 获取文件大小，若文件不存在则返回错误信息
fn file_size(file_path: &str) -> Result<u64, &str> {
    match std::fs::metadata(file_path) {
        Ok(metadata) => Ok(metadata.len()),
        Err(_) => Err("File not found!"),
    }
}

fn main() {
    // 调用倒计时函数
    count_down(5);

    // 尝试读取并打印 /etc/hosts 文件
    if let Err(e) = read_and_print("/etc/hosts") {
        println!("Error reading file: {}", e);
    }

    // 获取用户输入并尝试获取文件大小
    let mut input = String::new();
    println!("Enter file path to get its size:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let file_path = input.trim(); // 去除多余的换行符

    match file_size(file_path) {
        Ok(size) => println!("File size: {} bytes", size),
        Err(e) => println!("Error: {}", e),
    }
}
