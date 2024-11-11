use clap::{command, Command, Parser, Subcommand};

mod v0_0_1;

// #[derive(Debug, Clone, Parser)]
// pub struct VecArgs {
//     inner: Vec<String>,
// }

#[derive(Parser, Debug)]
pub struct Old {
    #[command(subcommand)]
    old_version: Version,
}

#[derive(Subcommand, Debug)]
enum Version {
    #[command(name = "0.0.1")]
    V0_0_1(v0_0_1::Args),
}

pub fn run(mut args: Vec<String>) -> std::io::Result<()> {
    // clap 的 parse 函数需要
    args.insert(0, "".to_string());

    if let Ok(args) = Old::try_parse_from(args.clone()) {
        match args.old_version {
            Version::V0_0_1(args) => v0_0_1::run(args),
        }
    } else {
        v0_0_1::run(v0_0_1::Args::parse_from(args))
    }
}
