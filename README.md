# Push BYCEPS Ticket Sale Stats to Discord

A tool to fetch ticket sale stats for a specific LAN party from a
[BYCEPS](https://byceps.nwsnet.de/) installation and show that as the
name of a channel on [Discord](https://discord.com/).

A Discord voice channel can be used for this purpose. After removing the
relevant access permissions for non-admin server users, it cannot be
joined and a lock is shown as its icon. This turns it into a small text
display.

The API of the BYCEPS installation has to be enabled and an API token
has to be configured and made available for the client.


## Usage

Create a configuration file based off of `config_example.toml`.

Run the program, pointing to it:

```sh
$ push-byceps-stats-to-discord config.toml
```


## License

This application is licensed under the MIT license.
