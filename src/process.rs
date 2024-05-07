use std::fs;

use csv::Reader;

use anyhow::Result;
use rand::prelude::*;
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
    let headers = reader.headers()?.clone();
    for result in reader.records() {
        let record = result?;
        let json_value = headers
            .iter()
            .zip(record.iter())
            .collect::<serde_json::Value>();
        ret.push(json_value);
    }

    let contents = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
        OutputFormat::Toml => {
            #[derive(Debug, Serialize)]
            struct Config {
                player: Vec<serde_json::Value>,
            }
            let config = Config { player: ret };
            toml::to_string_pretty(&config)?
        }
    };

    fs::write(format!("{}.{}", output, format), contents)?;
    Ok(())
}

// opts::GenPassOpts {
//     #[arg(short, long, default_value = "16")]
//     pub length: u8,
//     #[arg(short, long, default_value = "true")]
//     pub special: bool,
//     #[arg(short, long, default_value = "true")]
//     pub numbers: bool,
// }
pub fn generate_password(length: u8, special: &bool, numbers: &bool) -> Result<String> {
    let mut password = Vec::with_capacity(length as usize);
    let mut rng = rand::thread_rng();
    let mut chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let mut specials = "!@#$%^&*()_+-=[]{}|;:,.<>?".chars();
    let mut nums = "0123456789".chars();

    for _ in 0..length {
        let c = match (special, numbers) {
            (true, true) => match rng.gen_range(0..3) {
                0 => chars.next().unwrap(),
                1 => specials.next().unwrap(),
                _ => nums.next().unwrap(),
            },
            (true, false) => match rng.gen_range(0..2) {
                0 => chars.next().unwrap(),
                _ => specials.next().unwrap(),
            },
            (false, true) => match rng.gen_range(0..2) {
                0 => chars.next().unwrap(),
                _ => nums.next().unwrap(),
            },
            (false, false) => chars.next().unwrap(),
        };
        password.push(c);
    }
    password.shuffle(&mut rng);
    println!("{:?}", password.iter().collect::<String>());
    Ok(password.iter().collect())
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
            fs::read_to_string("fixtures/test2.json")?
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
            fs::read_to_string("fixtures/test2.yaml")?
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
            fs::read_to_string("fixtures/test2.toml")?
        );
        Ok(())
    }

    #[test]
    fn test_generate_password_by_length() -> Result<()> {
        let length = 16;
        let output = generate_password(length, &true, &true)?;
        // assert_eq!(stdout.length, 16)
        assert_eq!(output.len(), 16);
        Ok(())
    }

    #[test]
    fn test_generate_password_without_special() -> Result<()> {
        let length = 16;
        let output = generate_password(length, &false, &true)?;
        // assert_eq!(stdout.length, 16)
        // String包含字母和数字
        assert!(output
            .chars()
            .any(|c| "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c)));
        assert!(output.chars().any(|c| "0123456789".contains(c)));
        // 字符串不包含特殊字符
        assert!(!output
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)));
        Ok(())
    }

    #[test]
    fn test_generate_password_without_numbers() {
        let length = 16;
        let output = generate_password(length, &true, &false).unwrap();
        // String包含字母和特殊字符
        assert!(output
            .chars()
            .any(|c| "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".contains(c)));
        assert!(output
            .chars()
            .any(|c| "!@#$%^&*()_+-=[]{}|;:,.<>?".contains(c)));
        // 字符串不包含数字
        assert!(!output.chars().any(|c| "0123456789".contains(c)));
    }
}
