use mime::Mime;
use std::env;
use std::str::FromStr;

use serenity::async_trait;
use serenity::futures::prelude::*;
use serenity::json::Value;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::image as image_commands;
use crate::commands::{reddit, CommandResponse};

mod commands;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, bot_data: Ready) {
        println!("connected as {}", bot_data.user.name);

        command::Command::create_global_application_command(&ctx.http, |cmd| {
            cmd.name("reddit-random")
                .description("Get a random picture from the given subreddit")
                .create_option(|option| {
                    option
                        .name("subreddit")
                        .kind(command::CommandOptionType::String)
                        .description("Subreddit to get pictures from")
                        .required(true)
                })
        })
        .await
        .expect("Failed to register reddit-random command");

        command::Command::create_global_application_command(&ctx.http, |cmd| {
            cmd.name("nuke").description("Deep fry the last image")
        })
        .await
        .expect("Failed to register command");
    }

    async fn interaction_create(&self, ctx: Context, interaction: interaction::Interaction) {
        if let interaction::Interaction::ApplicationCommand(command) = interaction {
            println!("Received command {}", command.data.name);
            if let Err(e) = command.defer(&ctx.http).await {
                eprintln!("Failed to respond: {}", e);
                return;
            }

            let response_details = match command.data.name.as_str() {
                "reddit-random" => {
                    let opt_subreddit = command.data.options.get(0).map(|op| &op.value);

                    if let Some(Some(subreddit)) = opt_subreddit {
                        let subreddit_str = match subreddit {
                            Value::Bool(v) => v.to_string(),
                            Value::Number(v) => v.to_string(),
                            Value::String(v) => v.to_string(),
                            _ => unreachable!(),
                        };

                        reddit::random_image(subreddit_str).await
                    } else {
                        Ok(CommandResponse {
                            text: Some("Wat, send a subreddit you dick".to_string()),
                            ..Default::default()
                        })
                    }
                }
                "nuke" => {
                    let res = Box::pin(
                        command
                            .channel_id
                            .messages_iter(&ctx.http)
                            .filter(|res| future::ready(res.is_ok()))
                            .map(|res| res.unwrap())
                            .take(20)
                            // Find messages with either an image attachment or an image embed
                            .filter(|msg: &Message| {
                                future::ready(
                                    msg.attachments.iter().any(|a| {
                                        a.content_type
                                            .as_ref()
                                            .filter(|c| filter_image_mine(c))
                                            .is_some()
                                    }) || msg.embeds.iter().any(|embed| embed.image.is_some()),
                                )
                            }),
                    )
                    .next()
                    .await;

                    if let Some(msg) = res {
                        let url = msg.attachments
                            .iter()
                            .filter(|a| a.content_type.as_ref().filter(|c| filter_image_mine(c)).is_some())
                            .map(|a| &a.url)
                            .next()
                            .or_else(|| msg.embeds.iter().filter_map(|e| e.image.as_ref()).map(|ei| &ei.url).next())
                            .expect("Stuart assumed the message filter would ensure the message would have an image").to_string();

                        image_commands::nuke(url).await
                    } else {
                        Ok(CommandResponse {
                            text: Some(
                                "Bruv, looked 20 messages back, no fucking images".to_string(),
                            ),
                            ..Default::default()
                        })
                    }
                }
                _ => Ok(CommandResponse {
                    text: Some("Wat, use a real command".to_string()),
                    ..Default::default()
                }),
            };

            let result = command.create_followup_message(&ctx.http, |data| {
                if let Err(err) = response_details {
                    return data.content(format!("Failed to run command: {}", err));
                }

                let details = response_details.unwrap();
                if let Some(text) = details.text {
                    data.content(text);
                }
                if let Some(embed) = details.embed {
                    data.add_embed(embed);
                }
                if let Some(attachment) = details.attachment {
                    data.add_file(attachment);
                }

                data
            });

            if let Err(err) = result.await {
                eprintln!("Failed to respond to command: {}", err);
                command
                    .create_followup_message(&ctx.http, |data| {
                        data.content(
                            format!("On today's episode of how fucked up is fucked up, that's fucked up: {}", err),
                        )
                    })
                    .await
                    .expect("We really done fucked up");
            }
        }
    }
}

fn filter_image_mine(m: &str) -> bool {
    if let Ok(mime_val) = Mime::from_str(m) {
        mime_val.type_() == mime::IMAGE
    } else {
        false
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Token not found in env");

    let mut client = Client::builder(
        token,
        GatewayIntents::default() | GatewayIntents::MESSAGE_CONTENT,
    )
    .event_handler(Handler)
    .await
    .expect("Error creating client");

    if let Err(err) = client.start().await {
        eprintln!("Client error {}", err);
    }
}
