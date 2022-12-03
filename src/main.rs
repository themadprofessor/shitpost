use std::env;
use std::str::FromStr;

use mime::Mime;
use poise::serenity_prelude as serenity;
use tracing::log::Level;
use tracing_subscriber::{filter, EnvFilter};

mod commands;

#[derive(Debug)]
pub struct Data;

type Ctx<'a> = poise::Context<'a, Data, anyhow::Error>;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();
    let token = env::var("DISCORD_TOKEN").expect("Token not found in env");

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::get_commands(),
            ..Default::default()
        })
        .token(token)
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data)
            })
        });

    if let Err(err) = framework.run().await {
        eprintln!("Client error: {}", err);
    }
}
