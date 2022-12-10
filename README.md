# Shitpost
Discord shitposting bot

## Build

```shell
cargo build --release
```

## Running

The bot doesn't fork BTW.
```shell
shitpost
```

Or use the systemd service
```shell
cp shitpost /usr/bin/shitpost
cp shitpost.service /etc/systemd/system
systemctl start shitpost
```

## Config

See [Discord Applications](https://discord.com/developers/applications) to get your token.

The bot merges config from the config file (if present), command line arguments and the environment variables, in that order.

### Config file

`shitpost.toml` is an example config, so use it.
Depending on your system, the config is expected to be in different locations:
```
Linux    $XDG_CONFIG_HOME/shitpost/shitpost.toml
Windows  {FOLDERID_RoamingAppData}/shitpost/shitpost.toml
MacOS    $HOME/Library/Application Support/io.shitty.shitpost/shitpost.toml
```
On linux, if `$XDG_CONFIG_HOME` is not set, `$HOME/.config` is used instead.+

If `--config-file` is passed as a command line parameter, the file at the given path is merged in after the config file
specified above if it exists.

### Command line arguments

Use `shitpost --help` for a complete list of arguments.

In general, they are the same as in the config file, but in kebab-case.
e.g. `discord_token = 'foo'` and `--discord-token=foo`

### Environment variables

In general, they are the same as in the config file, but in SCREAMING_SNAKE_CASE and prefixed with `SHITPOST_`.
e.g. `discord_token = 'foo'` and `SHITPOST_DISCORD_TOKEN=foo`