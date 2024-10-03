# Changelog


## Version 0.6.0 (unreleased)

- Updated to Rust 1.81.0.

- Updated clap to v4.5.19.

- Updated log to v0.4.22.

- Updated serde_with to v3.10.0.

- Updated time to work on Rust 1.80.0 and later.

- Publish Docker images for platform `linux/arm64` (in addition to
  `linux/amd64`).


## Version 0.5.1 (2023-10-12)

- Adjusted configuration test to changed value in exemplary
  configuration file.

- Update Docker base images to Rust version 1.73.0.


## Version 0.5.0 (2023-10-12)

- Removed `/api` from beginning of URL path to BYCEPS API.

- Updated clap to v4.4.6.

- Updated log to v0.4.20.

- Updated simple_logger to v4.2.0.

- Updated toml to v0.8.2.

- Updated ureq to v2.8.0.


## Version 0.4.2 (2023-03-18)

- Fixed image path in Compose file.


## Version 0.4.1 (2023-03-18)

- Fixed Docker build.


## Version 0.4.0 (2023-03-18)

- Implemented repeated fetching and submitting of ticket sale stats.

- Implemented logging.

- Added `Dockerfile` and Docker Compose file.

- Updated clap to v4.1.9 (from v2.34).

- Updated toml to v0.7.3 (from v0.5.8).

- Updated ureq to v2.6.2 (from v2.4.0).


## Version 0.3.0 (2021-12-28)

- Switched to Rust edition 2021.

- Updated clap to v2.34 (from v2.33.3).

- Updated ureq to v2.4.0 (from v2.2.0).

- Use `ureq::patch`.


## Version 0.2.0 (2021-10-19)

- Adjusted to BYCEPS API no longer expecting the token to be
  Base64-encoded.

- Removed dependency on `base64`.


## Version 0.1.0 (2021-10-13)

- First public release
