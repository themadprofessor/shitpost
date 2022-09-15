use serenity::builder::CreateEmbed;
use serenity::model::prelude::interaction::InteractionResponseType;

pub mod reddit;

#[derive(Default, Debug)]
pub struct CommandResponse {
    pub kind: Option<InteractionResponseType>,
    pub embed: Option<CreateEmbed>,
    pub text: Option<String>,
}
