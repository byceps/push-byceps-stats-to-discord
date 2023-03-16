/*
 * Copyright 2021-2023 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub(crate) struct Config {
    pub(crate) byceps: BycepsConfig,
    pub(crate) discord: DiscordConfig,
}

#[derive(Debug, Deserialize)]
pub(crate) struct BycepsConfig {
    pub(crate) api_host: String,
    pub(crate) api_token: String,
    pub(crate) party_id: String,
}

#[derive(Debug, Deserialize)]
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
