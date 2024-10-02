/*
 * Copyright 2021-2024 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use crate::config::BycepsConfig;
use anyhow::Result;
use log::info;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct TicketSaleStats {
    pub tickets_max: u16,
    pub tickets_sold: u16,
}

pub(crate) fn get_ticket_sale_stats(config: &BycepsConfig) -> Result<TicketSaleStats> {
    let url = format!(
        "{}/v1/ticketing/sale_stats/{}",
        config.api_host, config.party_id
    );
    let authz_value = format!("Bearer {}", config.api_token);

    let request = ureq::get(&url).set("Authorization", &authz_value);
    let response = request.call()?;

    let stats = response.into_json::<TicketSaleStats>()?;
    info!(
        "Ticket stats: {} of {} sold",
        stats.tickets_sold, stats.tickets_max
    );

    Ok(stats)
}
