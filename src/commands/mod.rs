use std::future::Future;

use poise::serenity_prelude as serenity;
use poise::Command;
use serenity::builder::CreateEmbed;
use serenity::model::channel::AttachmentType;
use serenity::model::prelude::interaction::InteractionResponseType;

pub mod cope;
pub mod image;
pub mod reddit;

pub fn get_commands() -> Vec<Command<crate::Config, anyhow::Error>> {
    vec![image::nuke(), reddit::random_image(), cope::cope()]
}
