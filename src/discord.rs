/*
 * Copyright 2021-2023 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use crate::config::DiscordConfig;
use anyhow::Result;

pub(crate) fn set_channel_name(config: DiscordConfig, name: &str) -> Result<()> {
    let url = format!("https://discord.com/api/v9/channels/{}", config.channel_id);
    let authz_value = format!("Bot {}", config.bot_token);

    let request = ureq::patch(&url).set("Authorization", &authz_value);
    request.send_json(ureq::json!({ "name": name }))?;

    Ok(())
}
