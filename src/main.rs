/*
 * Copyright 2021-2023 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use anyhow::Result;
use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};
use serde::Deserialize;
use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Deserialize)]
struct Config {
    byceps: BycepsConfig,
    discord: DiscordConfig,
}

#[derive(Debug, Deserialize)]
struct BycepsConfig {
    api_host: String,
    api_token: String,
    party_id: String,
}

#[derive(Debug, Deserialize)]
struct DiscordConfig {
    bot_token: String,
    channel_id: String,
}

#[derive(Debug, Deserialize)]
struct TicketSaleStats {
    tickets_max: u16,
    tickets_sold: u16,
}

fn parse_args() -> ArgMatches<'static> {
    App::new(crate_name!())
        .author(crate_authors!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(
            Arg::with_name("config")
                .help("Specify configuration file")
                .required(true),
        )
        .get_matches()
}

fn load_config(path: &Path) -> Result<Config> {
    let text = read_to_string(path)?;
    let config: Config = toml::from_str(&text)?;
    Ok(config)
}

fn get_ticket_sale_stats(config: BycepsConfig) -> Result<TicketSaleStats> {
    let url = format!(
        "{}/api/v1/ticketing/sale_stats/{}",
        config.api_host, config.party_id
    );
    let authz_value = format!("Bearer {}", config.api_token);

    let request = ureq::get(&url).set("Authorization", &authz_value);
    let response = request.call()?;

    let stats = response.into_json::<TicketSaleStats>()?;

    Ok(stats)
}

fn set_discord_channel_name(config: DiscordConfig, name: &str) -> Result<()> {
    let url = format!("https://discord.com/api/v9/channels/{}", config.channel_id);
    let authz_value = format!("Bot {}", config.bot_token);

    let request = ureq::patch(&url).set("Authorization", &authz_value);
    request.send_json(ureq::json!({ "name": name }))?;

    Ok(())
}

fn main() -> Result<()> {
    let args = parse_args();

    let config_filename = args.value_of("config").map(Path::new).unwrap();
    let config = load_config(config_filename)?;

    let stats = get_ticket_sale_stats(config.byceps)?;
    let channel_name = format!(
        "Tickets sold: {} / {}",
        stats.tickets_sold, stats.tickets_max
    );
    set_discord_channel_name(config.discord, &channel_name)?;

    Ok(())
}
