/*
 * Copyright 2021-2023 Jochen Kupperschmidt
 * License: MIT (see file `LICENSE` for details)
 */

use log::Level;

pub(crate) fn configure(level: Level) {
    simple_logger::init_with_level(level).unwrap()
}

pub(crate) fn get_default_log_level() -> Level {
    Level::Error
}
