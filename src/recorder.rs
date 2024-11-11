use clap::{command, Parser};
use crossbeam::channel::bounded;
use std::fs::OpenOptions;
use std::io::{BufWriter, Write};
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};
use std::{thread, time};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    // 监听ip
    #[arg(long, default_value = "0.0.0.0")]
    ip: IpAddr,
    // 监听端口
    #[arg(short, long)]
    port: u16,

    #[arg(short('s'), long, default_value_t = 2048)]
    channel_size: usize,

    #[arg(long, default_value_t = 65535)]
    buffer_size: usize,

    // 输出文件
    #[arg(short, long, default_value = "data")]
    output: PathBuf,
}

fn write_msg(writer: &mut BufWriter<std::fs::File>, msg: &[u8]) -> std::io::Result<()> {
    let length_prefix = msg.len() as u16; // 获取消息长度
    // println!("Message length: {}", length_prefix);

    // 写入长度前缀
    writer.write_all(&length_prefix.to_le_bytes())?;
    // 写入消息内容
    writer.write_all(msg)?;
    Ok(())
}

pub fn run(args: Args, ver: [u8; 3]) -> std::io::Result<()> {
    let addr = SocketAddr::new(args.ip, args.port);
    println!("Listen on {addr}.");
    // 绑定到指定的地址和端口
    let socket = UdpSocket::bind(addr)?;

    // // 创建一个通道用于线程间通信
    let (tx, rx) = bounded(args.channel_size);
    // let (tx, rx): (mpsc::Sender<Vec<u8>>, mpsc::Receiver<Vec<u8>>) = mpsc::channel();
    let ctrlc_flag = Arc::new(AtomicBool::new(false));
    let flag_clone = ctrlc_flag.clone();
    let flag_clone_tx = ctrlc_flag.clone();
    let flag_clone_rx = ctrlc_flag.clone();

    // 线程1：接收 UDP 数据
    let receiver_thread = thread::spawn(move || {
        let mut buf = [0; 65535];

        while !flag_clone_tx.load(Ordering::Relaxed) {
            let (size, _) = match socket.recv_from(&mut buf) {
                Ok(res) => res,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => continue,
                Err(e) => {
                    eprintln!("Error receiving data: {}", e);
                    continue;
                }
            };
            let msg = buf[..size].to_vec();
            // println!("Received message: {}", std::str::from_utf8(&msg).unwrap());
            if let Err(e) = tx.send(msg) {
                eprintln!("Failed to send message to writer thread.\n {e}");
            }
        }
    });

    // 线程2：写入文件

    // 避免覆盖
    if std::fs::exists(&args.output)? {
        println!("File has existed! Do you want to overwrite? (Y/n)");
        loop {
            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;
            if input.as_bytes()[0] == b'Y' {
                break;
            } else if input.as_bytes()[0] == b'n' {
                std::process::exit(0);
            }

            println!("File has existed! Do you want to overwrite? (Y/n)");
        }
    }

    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&args.output)?;

    // 创建一个 BufWriter，指定缓冲区大小（可选）
    let writer = Arc::new(Mutex::new(BufWriter::with_capacity(args.buffer_size, file)));

    ctrlc::set_handler(move || {
        // 设置 ctrlc_flag 为 true
        flag_clone.store(true, Ordering::Relaxed);
    })
    .expect("Error setting Ctrl-C handler");

    let writer_thread = thread::spawn({
        let writer = Arc::clone(&writer);
        move || {
            let mut writer = writer.lock().unwrap();

            // 写入文件版本号
            writer.write_all(&ver).unwrap();

            loop {
                match rx.try_recv() {
                    Ok(msg) => {
                        if let Err(e) = write_msg(&mut writer, &msg) {
                            eprintln!("Failed to write message: {}", e);
                        }
                    }
                    Err(_) => {
                        if flag_clone_rx.load(Ordering::Relaxed) {
                            writer.flush().unwrap();
                            println!("Buffer flushed on Ctrl+C.");
                            std::process::exit(0);
                        }

                        thread::sleep(time::Duration::from_micros(10));
                    }
                }
            }
        }
    });

    // 等待线程结束
    receiver_thread.join().unwrap();
    writer_thread.join().unwrap();

    Ok(())
}
