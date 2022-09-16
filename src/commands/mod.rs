use serenity::builder::CreateEmbed;
use serenity::model::channel::AttachmentType;
use serenity::model::prelude::interaction::InteractionResponseType;

pub mod image;
pub mod reddit;

#[derive(Default, Debug)]
pub struct CommandResponse<'a> {
    pub kind: Option<InteractionResponseType>,
    pub embed: Option<CreateEmbed>,
    pub text: Option<String>,
    pub attachment: Option<AttachmentType<'a>>,
}
