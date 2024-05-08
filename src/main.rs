use anyhow::Result;
use clap::Parser;
use rcli::{
    opts::{Base64Opts, CsvOpts, GenPassOpts},
    process::{generate_password, process_csv},
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "convert CSV to other formats")]
    Csv(CsvOpts),
    #[command(name = "gen-pass", about = "generate a random password")]
    GenPass(GenPassOpts),
    #[command(subcommand)]
    Base64(Base64Opts),
}

// rcli csv -i input.csv -o output.csv --header -d ','
fn main() -> Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output, &opts.format),
        SubCommand::GenPass(opts) => {
            generate_password(
                opts.length,
                opts.lower,
                opts.upper,
                opts.special,
                opts.numbers,
            )?;
            Ok(())
        }
        SubCommand::Base64(opts) => match opts {
            Base64Opts::Encode(opts) => rcli::process::encode_base64(&opts.input, &opts.output),
            Base64Opts::Decode(opts) => rcli::process::decode_base64(&opts.input, &opts.output),
        },
    }
}
