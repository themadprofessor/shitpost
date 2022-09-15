use std::env;

use serenity::async_trait;
use serenity::json::Value;
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::reddit::RedditCommand;
use crate::commands::CommandResponse;

mod commands;

#[derive(Default)]
struct Handler {
    capy: RedditCommand,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, bot_data: Ready) {
        println!("connected as {}", bot_data.user.name);

        command::Command::create_global_application_command(ctx.http.clone(), |cmd| {
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
        .expect("Failed to register commands");
    }

    async fn interaction_create(&self, ctx: Context, interaction: interaction::Interaction) {
        if let interaction::Interaction::ApplicationCommand(command) = interaction {
            println!("Received command {}", command.data.name);

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

                        self.capy.random_image(subreddit_str).await
                    } else {
                        Ok(CommandResponse {
                            text: Some("Wat, send a subreddit you dick".to_string()),
                            ..Default::default()
                        })
                    }
                }
                _ => Ok(CommandResponse {
                    text: Some("Wat, use a real command".to_string()),
                    ..Default::default()
                }),
            };

            let result =
                command.create_interaction_response(&ctx.http, |builder| {
                    if let Err(err) = response_details {
                        return builder
                            .kind(interaction::InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(|res_builder| {
                                res_builder.content(format!("Failed to run command: {}", err))
                            });
                    }

                    let details = response_details.unwrap();
                    builder
                        .kind(details.kind.unwrap_or(
                            interaction::InteractionResponseType::ChannelMessageWithSource,
                        ))
                        .interaction_response_data(|res_builder| {
                            if let Some(text) = details.text {
                                res_builder.content(text);
                            }
                            if let Some(embed) = details.embed {
                                res_builder.add_embed(embed);
                            }

                            res_builder
                        })
                });

            if let Err(err) = result.await {
                eprintln!("Failed to reponse to command: {}", err);
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Token not found in env");

    let mut client = Client::builder(token, GatewayIntents::default())
        .event_handler(Handler::default())
        .await
        .expect("Error creating client");

    if let Err(err) = client.start().await {
        eprintln!("Client error {}", err);
    }
}
