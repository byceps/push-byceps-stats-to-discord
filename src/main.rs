/*
 * Copyright 2021-2023 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use crate::config::load_config;
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

    let stats = byceps::get_ticket_sale_stats(config.byceps)?;

    let channel_name = format!(
        "Tickets sold: {} / {}",
        stats.tickets_sold, stats.tickets_max
    );
    discord::set_channel_name(config.discord, &channel_name)?;

    Ok(())
}
