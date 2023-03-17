/*
 * Copyright 2021-2023 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use crate::config::DiscordConfig;
use anyhow::Result;
use log::error;
use log::info;

pub(crate) fn set_channel_name(config: DiscordConfig, name: &str) -> Result<()> {
    let url = format!("https://discord.com/api/v9/channels/{}", config.channel_id);
    let authz_value = format!("Bot {}", config.bot_token);

    let request = ureq::patch(&url).set("Authorization", &authz_value);
    match request.send_json(ureq::json!({ "name": name })) {
        Ok(_) => info!("Discord channel name updated."),
        Err(ureq::Error::Status(code, response)) => {
            error!(
                "Discord channel name update failed: {} {}",
                code, response.status_text()
            )
        }
        Err(_) => error!("Discord channel name update failed due to I/O or transport error."),
    }

    Ok(())
}
