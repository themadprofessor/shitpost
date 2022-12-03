use poise::serenity_prelude as serenity;
use poise::Command;
use serenity::builder::CreateEmbed;
use serenity::model::channel::AttachmentType;
use serenity::model::prelude::interaction::InteractionResponseType;
use std::future::Future;

pub mod image;
pub mod reddit;

pub fn get_commands() -> Vec<Command<crate::Data, anyhow::Error>> {
    vec![image::nuke(), reddit::random_image()]
}
