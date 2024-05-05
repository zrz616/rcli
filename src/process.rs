use std::fs;

use csv::Reader;

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::opts::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub(crate) name: String,
    pub(crate) position: String,
    #[serde(rename = "DOB")]
    pub(crate) dob: String,
    pub(crate) nationality: String,
    #[serde(rename = "Kit Number")]
    pub(crate) kit: u8,
}

pub fn process_csv(input: &str, output: &str, format: &OutputFormat) -> Result<()> {
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
            #[derive(Debug, Serialize)]
            struct Config {
                player: Vec<Player>,
            }
            let config = Config { player: ret };
            toml::to_string_pretty(&config)?
        }
    };

    fs::write(format!("{}.{}", output, format), contents)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_csv_json() -> Result<()> {
        let input = "assets/juventus.csv";
        let output = "output";
        let format = OutputFormat::Json;
        process_csv(input, output, &format)?;
        assert_eq!(
            fs::read_to_string("output.json")?,
            fs::read_to_string("fixtures/test.json")?
        );
        Ok(())
    }

    #[test]
    fn test_process_csv_yaml() -> Result<()> {
        let input = "assets/juventus.csv";
        let output = "output";
        let format = OutputFormat::Yaml;
        process_csv(input, output, &format)?;
        assert_eq!(
            fs::read_to_string("output.yaml")?,
            fs::read_to_string("fixtures/test.yaml")?
        );
        Ok(())
    }

    #[test]
    fn test_process_csv_toml() -> Result<()> {
        let input = "assets/juventus.csv";
        let output = "output";
        let format = OutputFormat::Toml;
        process_csv(input, output, &format)?;
        assert_eq!(
            fs::read_to_string("output.toml")?,
            fs::read_to_string("fixtures/test.toml")?
        );
        Ok(())
    }
}
