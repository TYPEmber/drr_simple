use std::{ffi::OsString, str::FromStr};

use clap::{command, Parser, Subcommand};

mod v0_0_1;

#[derive(Parser, Debug)]
pub struct ReplayerArgs {
    #[command(subcommand)]
    version: Version
}

#[derive(Subcommand, Debug)]
pub enum Version {
    #[command(name = "latest")]
    Latest(v0_0_1::Args),
    #[command(name = "0.0.1")]
    V0_0_1(v0_0_1::Args),
}

// 预处理版本号
pub fn pre_proc(args_os: std::env::ArgsOs) -> impl Iterator<Item = OsString> {
    args_os.flat_map(|arg| {
        if let Some(arg_s) = arg.to_str() {
            let arg_s = if arg_s == "replay" {
                "replay@latest"
            } else {
                arg_s
            };
            
            if arg_s.starts_with("replay@") {
                arg_s
                    .split('@')
                    .map(|s| OsString::from_str(s).unwrap())
                    .collect()
            } else {
                vec![arg]
            }
        } else {
            vec![arg]
        }
    })
}

// 设定最新版本
use v0_0_1 as LatestVersion;

pub fn run(args: ReplayerArgs) -> std::io::Result<()> {
    match args.version {
        Version::Latest(args) => LatestVersion::run(args),
        Version::V0_0_1(args) =>  v0_0_1::run(args),
    }
}
