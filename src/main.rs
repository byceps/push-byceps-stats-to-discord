/*
 * Copyright 2021-2023 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use crate::config::{load_config, BycepsConfig, Config, DiscordConfig};
use anyhow::Result;
mod byceps;
mod cli;
mod config;
mod discord;
mod logging;

fn main() -> Result<()> {
    let cli = cli::parse();

    let config = load_config(&cli.config_filename)?;

    logging::configure(config.log_level);

    let app = Application::new(config);

    app.run()?;

    Ok(())
}

struct Application {
    byceps_config: BycepsConfig,
    discord_config: DiscordConfig,
}

impl Application {
    fn new(config: Config) -> Self {
        Self {
            byceps_config: config.byceps,
            discord_config: config.discord,
        }
    }

    fn run(&self) -> Result<()> {
        let stats = byceps::get_ticket_sale_stats(&self.byceps_config)?;

        let channel_name = format!(
            "Tickets sold: {} / {}",
            stats.tickets_sold, stats.tickets_max
        );
        discord::set_channel_name(&self.discord_config, &channel_name)?;

        Ok(())
    }
}
