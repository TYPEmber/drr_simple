use std::net::UdpSocket;
use std::fs::File;
use std::io::{self, Read};
use std::str;

fn main() -> std::io::Result<()> {
    // 创建 UDP 套接字
    let socket = UdpSocket::bind("0.0.0.0:0")?; // 绑定到任意可用端口
    let target = "127.0.0.1:12345"; // 本地地址与接收端口

    let message = b"Hello, hwh/";

    // 发送数据
    socket.send_to(message, target)?;
    println!("Sent message: {:?}", std::str::from_utf8(message).unwrap());

    Ok(())
}
