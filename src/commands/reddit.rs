use crate::Ctx;
use anyhow::{bail, Result};
use poise::serenity_prelude::CreateEmbed;
use poise::CreateReply;
use rand::prelude::*;
use roux::util::{FeedOption, TimePeriod};
use roux::Subreddit;
use tracing::{debug, instrument, span};

#[derive(Debug, poise::ChoiceParameter)]
pub enum Ordering {
    Hot,
    Rising,
    Top,
    Latest,
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum Period {
    Now,
    Today,
    ThisWeek,
    ThisMonth,
    ThisYear,
    AllTime,
}

impl Into<TimePeriod> for Period {
    fn into(self) -> TimePeriod {
        match self {
            Period::Now => TimePeriod::Now,
            Period::Today => TimePeriod::Today,
            Period::ThisWeek => TimePeriod::ThisWeek,
            Period::ThisMonth => TimePeriod::ThisMonth,
            Period::ThisYear => TimePeriod::ThisYear,
            Period::AllTime => TimePeriod::AllTime,
        }
    }
}

/// Get a random image from the top 20 posts of a given subreddit this week.
#[poise::command(slash_command)]
#[instrument(skip(ctx), name = "random_image")]
pub async fn random_image(
    ctx: Ctx<'_>,
    #[description = "Subreddit to get the image from. No / or r/ plz"] subreddit: String,
    #[description = "Ordering of posts to pick from. Defaults to top"] ordering: Option<Ordering>,
    #[description = "Number of posts to fetch to pick from between 1 and 255. Defaults to 20"]
    #[max = 255_u8]
    #[min = 1_u8]
    count: Option<u8>,
    #[description = "Time period to pick from. Defaults to this week"] period: Option<Period>,
) -> Result<()> {
    if subreddit.contains("/") {
        bail!("Bruh, I said no /");
    }
    ctx.defer().await?;

    let result = {
        let subreddit = Subreddit::new(&subreddit);
        let count = count.unwrap_or(20);
        let options =
            Some(FeedOption::new().period(period.map(Into::into).unwrap_or(TimePeriod::ThisWeek)));
        match ordering.unwrap_or(Ordering::Top) {
            Ordering::Hot => subreddit.hot(count as u32, options).await,
            Ordering::Rising => subreddit.rising(count as u32, options).await,
            Ordering::Top => subreddit.top(count as u32, options).await,
            Ordering::Latest => subreddit.latest(count as u32, options).await,
        }?
    };
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
