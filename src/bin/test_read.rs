use std::fs::File;
use std::io::{BufReader, Read, Result};

fn read_messages_from_file(file_path: &str) -> Result<Vec<String>> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut messages = Vec::new();

    loop {
        // 用于存储长度前缀的 4 字节数组
        let mut length_bytes = [0u8; 4];
        // 读取长度前缀
        let bytes_read = reader.read_exact(&mut length_bytes);

        // 检查是否到达文件末尾
        match bytes_read {
            Ok(_) => {
                // 将长度前缀转换为 u32
                let length = u32::from_le_bytes(length_bytes); 
                let mut msg_bytes = vec![0u8; length as usize];
                // 读取消息内容
                reader.read_exact(&mut msg_bytes)?;
                // 将字节转换为字符串
                let message = String::from_utf8(msg_bytes).expect("Invalid UTF-8 sequence");
                messages.push(message);
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::UnexpectedEof => break, // 到达文件末尾
            Err(e) => return Err(e), // 处理其他错误
        }
    }
    Ok(messages)
}

fn main() -> Result<()> {
    let file_path = "output"; // 替换为你的文件路径
    
    // 从文件读取消息
    match read_messages_from_file(file_path) {
        Ok(messages) => {
            println!("Read messages:");
            for message in messages {
                println!("{}", message);
            }
        }
        Err(e) => eprintln!("Error reading messages: {}", e),
    }

    Ok(())
}