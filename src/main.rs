use std::{ffi::OsString, io::Read, str::FromStr};

use chrono::format::Item;
use clap::{command, Parser, Subcommand};

mod recorder;
mod replayer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    Record(recorder::Args),
    Replay(replayer::ReplayerArgs),
    Glance {
        #[arg(default_value = "data")]
        path: std::path::PathBuf,
    },
}

fn main() -> std::io::Result<()> {
    // 获取版本号
    // 限定为 3 个 u8 表达版本号
    let ver = env!("CARGO_PKG_VERSION");

    let ver = ver
        .split('.')
        .map(|c| c.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(ver.len(), 3);

    let ver = [ver[0], ver[1], ver[2]];

    // 预处理器
    let args_os = std::env::args_os();
    let args_os = replayer::pre_proc(args_os);

    // 获取命令行参数
    let args = Args::parse_from(args_os);

    match args.command {
        Command::Record(args) => recorder::run(args, ver),
        Command::Replay(args) => replayer::run(args),
        Command::Glance { path } => {
            let mut ver = [0u8; 3];
            std::fs::File::open(&path)?.read_exact(&mut ver)?;
            println!(
                "{}'s file version is: {}.{}.{}",
                path.canonicalize()?.to_str().unwrap(),
                ver[0],
                ver[1],
                ver[2]
            );
            Ok(())
        }
    }
}
