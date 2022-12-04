use anyhow::{Context, Result};
use clap::Parser;
use figment::providers::{Env, Format, Serialized, Toml};
use figment::Figment;
use std::env;
use std::str::FromStr;

use mime::Mime;
use poise::serenity_prelude as serenity;
use serde::{Deserialize, Serialize};
use tracing::info;
use tracing::log::Level;
use tracing_subscriber::{filter, EnvFilter};

mod commands;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Config {
    discord_token: String,
}

#[derive(Debug, Default, Serialize, Deserialize, Parser)]
#[command(author, version, about)]
struct Arguments {
    /// Discord API token
    #[arg(short, long, value_name = "TOKEN")]
    discord_token: Option<String>,
}

type Ctx<'a> = poise::Context<'a, Config, anyhow::Error>;

impl Config {
    fn new() -> Result<Self> {
        let dirs = directories::ProjectDirs::from("io", "shitty", "shitpost")
            .context("can't find home dir")?;
        Figment::new()
            .merge(Toml::file(dirs.config_dir().join("shitpost.toml")))
            .merge(Serialized::defaults(Arguments::parse()))
            .merge(Env::prefixed("SHITPOST_"))
            .extract()
            .context("failed to deserialize config data")
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .compact()
        .init();
    let cfg = Config::new()?;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            ..Default::default()
        })
        .token(&cfg.discord_token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(cfg)
            })
        });

    framework.run().await.context("client error")
}
