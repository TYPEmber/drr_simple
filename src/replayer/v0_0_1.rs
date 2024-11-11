use clap::Parser;

use std::io::BufReader;
use std::time::Duration;
use std::{fs::File, io::Seek, net::UdpSocket};
use std::{
    io::Read,
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

#[derive(Parser, Debug, Clone)]
/// Ver 0.0.1
/// 
/// Memory Layout
/// Version: [u8; 3]
/// [(LEN, MSG); N]: [(u16, [u8; LEN]); N]
#[command(verbatim_doc_comment)]
pub struct Args {
    // 监听ip
    #[arg(long, default_value = "127.0.0.1")]
    ip: IpAddr,
    // 监听端口
    #[arg(short, long)]
    port: u16,

    #[arg(short('c'), long, default_value_t = 100)]
    interval_count: usize,
    #[arg(short('s'), long, default_value_t = 1e-3)]
    interval_seconds: f64,
    // 循环播放
    #[arg(short, long)]
    r#loop: bool,

    // 输出文件
    #[arg(short, long, default_value = "data")]
    input: PathBuf,
}

pub fn run(args: Args) -> std::io::Result<()> {
    println!(
        "{}",
        clap::builder::StyledStr::from("\x1b[10mThis is a styled message.\x1b[0m").ansi()
    );
    let mut data = BufReader::new(std::fs::File::open(&args.input)?);
    // 确认版本号
    let mut ver = [0u8; 3];
    data.read_exact(&mut ver)?;
    assert_eq!(ver, [0, 0, 1]);

    let addr = SocketAddr::new(args.ip, args.port);
    println!("Send to {addr}.");
    // 绑定到指定的地址和端口
    let socket = UdpSocket::bind("0.0.0.0:0")?;

    loop {
        if let Err(e) = main_loop(&args, &mut data, &socket, addr) {
            if std::io::ErrorKind::UnexpectedEof == e.kind() && args.r#loop {
                data.seek(std::io::SeekFrom::Start(ver.len() as u64))?;
            } else {
                break Err(e);
            }
        }
    }
}

fn main_loop(
    args: &Args,
    data: &mut BufReader<File>,
    socket: &UdpSocket,
    addr: SocketAddr,
) -> std::io::Result<()> {
    let mut counter = 0usize;
    let dur = Duration::from_micros((args.interval_seconds * 1e6) as u64);

    let mut length = 0u16.to_le_bytes();
    let mut buffer = [0u8; u16::MAX as usize];

    loop {
        data.read_exact(&mut length)?;

        let len = u16::from_le_bytes(length) as usize;
        data.read_exact(&mut buffer[..len])?;

        socket.send_to(&buffer[..len], addr)?;

        // 控制发送速度
        {
            counter += 1;
            if counter % args.interval_count == 0 {
                std::thread::sleep(dur);
            }
        }
    }
}
