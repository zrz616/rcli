use anyhow::{Error, Result};
use clap::Parser;
use std::{fmt, path::Path, str::FromStr};

#[derive(Debug, Parser)]
pub struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,

    #[arg(short, long, default_value = "output")]
    pub output: String,

    #[arg(long, default_value_t = false)]
    pub header: bool,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    pub format: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
pub enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value = "16")]
    pub length: u8,
    #[arg(long, default_value = "true")]
    pub lower: bool,
    #[arg(long, default_value = "true")]
    pub upper: bool,
    #[arg(long, default_value_t = false)]
    pub special: bool,
    #[arg(long, default_value_t = false)]
    pub numbers: bool,
}

#[derive(Debug, Parser)]
pub enum Base64Opts {
    #[command(name = "encode", about = "encode a file to base64")]
    Encode(EncodeOpts),
    #[command(name = "decode", about = "decode a base64 file")]
    Decode(DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct EncodeOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output")]
    pub output: String,
}

#[derive(Debug, Parser)]
pub struct DecodeOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    #[arg(short, long, default_value = "output")]
    pub output: String,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if Path::new(filename).exists() || filename == "-" {
        Ok(filename.into())
    } else {
        Err("File does not exist")
    }
}

fn parse_format(format: &str) -> Result<OutputFormat, Error> {
    format.parse()
}

impl From<OutputFormat> for &'static str {
    fn from(format: OutputFormat) -> &'static str {
        match format {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
            OutputFormat::Toml => "toml",
        }
    }
}

impl FromStr for OutputFormat {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            "toml" => Ok(OutputFormat::Toml),
            _ => Err(Error::msg("Invalid format")),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", Into::<&'static str>::into(*self))
    }
}
