use anyhow::{Error, Result};
use clap::Parser;
use csv::Reader;
use serde::{Deserialize, Serialize};
use std::{fmt, fs, str::FromStr};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[command(name = "csv", about = "Show CSV, or convert CSV to other formats")]
    Csv(CsvOpts),
}

#[derive(Debug, Parser)]
struct CsvOpts {
    #[arg(short, long, value_parser = verify_input_file)]
    input: String,

    #[arg(short, long, default_value = "output")]
    output: String,

    #[arg(long, default_value_t = false)]
    header: bool,

    #[arg(short, long, default_value_t = ',')]
    delimiter: char,

    #[arg(short, long, value_parser = parse_format, default_value = "json")]
    format: OutputFormat,
}

#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Json,
    Yaml,
    Toml,
}

fn verify_input_file(filename: &str) -> Result<String, &'static str> {
    if std::path::Path::new(filename).exists() {
        Ok(filename.into())
    } else {
        Err("Input file does not exist")
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

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    name: String,
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    player: Vec<Player>,
}

fn process_csv(input: &str, output: &str, format: &OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128);
    for result in reader.deserialize() {
        let record: Player = result?;
        ret.push(record);
    }

    let contents = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => {
            let config = Config { player: ret };
            println!("{:?}", config);
            toml::to_string_pretty(&config)?
        }
    };

    fs::write(format!("{}.{}", output, format), contents)?;
    Ok(())
}

// rcli csv -i input.csv -o output.csv --header -d ','
fn main() -> Result<()> {
    let opts = Opts::parse();
    println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => process_csv(&opts.input, &opts.output, &opts.format),
    }
}
