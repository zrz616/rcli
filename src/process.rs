use std::{fs, io::stdin, io::Read};

use base64::prelude::*;
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
const SPECIALS: &[u8] = b"!@#$%^&*()_+-=[]{}|;:,.<>?";
const NUMS: &[u8] = b"0123456789";
const LOWER: &[u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPER: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
pub fn generate_password(
    length: u8,
    lower: bool,
    upper: bool,
    special: bool,
    numbers: bool,
) -> Result<String> {
    let mut password = Vec::with_capacity(length as usize);
    let mut chars = Vec::new();
    let mut rng = rand::thread_rng();

    if special {
        chars.extend_from_slice(SPECIALS);
        password.push(*SPECIALS.choose(&mut rng).unwrap());
    }
    if numbers {
        chars.extend_from_slice(NUMS);
        password.push(*NUMS.choose(&mut rng).unwrap());
    }
    if lower {
        chars.extend_from_slice(LOWER);
        password.push(*LOWER.choose(&mut rng).unwrap());
    }
    if upper {
        chars.extend_from_slice(UPPER);
        password.push(*UPPER.choose(&mut rng).unwrap());
    }

    for _ in 0..(length - password.len() as u8) {
        let c = chars.choose(&mut rng).unwrap();
        password.push(*c);
    }

    password.shuffle(&mut rng);
    println!("{:?}", String::from_utf8(password.clone()));
    Ok(String::from_utf8(password)?)
}

pub fn encode_base64(input: &str, output: &str) -> Result<()> {
    if input == "-" {
        let mut buffer = Vec::new();
        stdin().read_to_end(&mut buffer)?;
        let encoded = BASE64_STANDARD.encode(buffer);
        fs::write(output, encoded)?;
        println!("{:?}", fs::read_to_string(output)?);
        return Ok(());
    }
    let contents = fs::read(input)?;
    let encoded = BASE64_STANDARD.encode(contents);
    fs::write(output, encoded)?;
    Ok(())
}

pub fn decode_base64(input: &str, output: &str) -> Result<()> {
    let contents = fs::read(input)?;
    let decoded = BASE64_STANDARD.decode(contents)?;
    fs::write(output, decoded)?;
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
        let output = generate_password(length, true, true, true, true)?;
        // assert_eq!(stdout.length, 16)
        assert_eq!(output.len(), 16);
        Ok(())
    }

    #[test]
    fn test_generate_password_without_special() -> Result<()> {
        let length = 16;
        let output = generate_password(length, true, true, false, true)?;
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
        let output = generate_password(length, true, true, true, false).unwrap();
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

    #[test]
    fn test_encode_by_input_file() -> Result<()> {
        encode_base64("assets/juventus.csv", "output")?;
        assert_eq!(
            fs::read_to_string("output")?,
            fs::read_to_string("fixtures/test.b64")?
        );
        Ok(())
    }
}
