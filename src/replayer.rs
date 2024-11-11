use clap::{command, CommandFactory, Parser, Subcommand};

mod v0_0_1;

// 设定最新版本
use v0_0_1 as LatestVersion;
use v0_0_1 as DefaultVersion;

#[derive(Parser, Debug)]
pub struct ReplayerArgs {
    #[command(subcommand)]
    version: Version,
}

#[derive(Subcommand, Debug)]
pub enum Version {
    #[command(name = "replay")]
    Defualt(DefaultVersion::Args),
    #[command(name = "replay@latest")]
    Latest(LatestVersion::Args),
    #[command(name = "replay@0.0.1")]
    V0_0_1(v0_0_1::Args),
}

pub fn run(args: Version) -> std::io::Result<()> {
    match args {
        Version::Defualt(args) => DefaultVersion::run(args),
        Version::Latest(args) => LatestVersion::run(args),
        Version::V0_0_1(args) => v0_0_1::run(args),
    }
}

pub fn get_ver_about(ver: [u8; 3]) -> String {
    match ver {
        [0, 0, 1] => v0_0_1::Args::command().get_about().unwrap().to_string(),
        _ => unreachable!(),
    }
}
