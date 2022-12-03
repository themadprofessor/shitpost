use crate::Ctx;
use anyhow::{bail, Result};
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use rand::prelude::*;
use roux::util::{FeedOption, TimePeriod};
use roux::Subreddit;
use tracing::{debug, instrument, span};

/// Get a random image from the top 20 posts of a given subreddit.
#[poise::command(slash_command)]
#[instrument]
pub async fn random_image(
    ctx: Ctx<'_>,
    #[description = "Subreddit to get the image from. No / or r/ plz"] subreddit: String,
) -> Result<()> {
    if subreddit.contains("/") {
        bail!("Bruh, I said no /");
    }
    ctx.defer().await?;

    let result = Subreddit::new(&subreddit)
        .top(20, Some(FeedOption::new().period(TimePeriod::ThisWeek)))
        .await?;
    debug!(subreddit, result_count = result.data.children.len());

    let pos_capy = result
        .data
        .children
        .iter()
        .map(|thing| &thing.data)
        .filter_map(|post| post.url.as_ref())
        .filter(|post| {
            !post.contains("gallery") && !post.contains("comments") && !post.contains("v.redd")
        })
        .choose(&mut thread_rng());

    if let Some(capy) = pos_capy {
        ctx.send(|f: &mut CreateReply| f.embed(|e: &mut CreateEmbed| e.image(capy)))
            .await?;
    } else {
        ctx.say("Bruh, we ain't found shit").await?;
    }
    Ok(())
}
