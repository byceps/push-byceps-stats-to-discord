/*
 * Copyright 2021-2023 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use clap::{crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches};

pub(crate) fn parse_args() -> ArgMatches<'static> {
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
