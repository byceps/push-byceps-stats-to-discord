/*
 * Copyright 2021-2023 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use crate::logging::get_default_log_level;
use anyhow::Result;
use log::Level;
use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr, DurationSeconds};
use std::fs::read_to_string;
use std::path::Path;
use std::time::Duration;

#[serde_as]
#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct Config {
    #[serde(default = "get_default_log_level")]
    #[serde_as(as = "DisplayFromStr")]
    pub(crate) log_level: Level,

    pub(crate) byceps: BycepsConfig,
    pub(crate) discord: DiscordConfig,

    #[serde(rename = "interval_in_seconds")]
    #[serde_as(as = "Option<DurationSeconds<u64>>")]
    pub(crate) interval: Option<Duration>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct BycepsConfig {
    pub(crate) api_host: String,
    pub(crate) api_token: String,
    pub(crate) party_id: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub(crate) struct DiscordConfig {
    pub(crate) bot_token: String,
    pub(crate) channel_id: String,
}

/// Load configuration from TOML file.
pub(crate) fn load_config(path: &Path) -> Result<Config> {
    let text = read_to_string(path)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load_config() {
        let expected = Config {
            log_level: Level::Error,
            byceps: BycepsConfig {
                api_host: "https://byceps.example".to_owned(),
                api_token: "your-secret-api-token".to_owned(),
                party_id: "acmeparty-2021".to_owned(),
            },
            discord: DiscordConfig {
                bot_token: "your-secret-bot-token".to_owned(),
                channel_id: "123456789012345678".to_owned(),
            },
            interval: None,
        };

        let actual = load_config(Path::new("config_example.toml"));
        assert!(actual.is_ok());
        assert_eq!(actual.unwrap(), expected);
    }
}
