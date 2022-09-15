use crate::commands::CommandResponse;
use rand::prelude::*;
use roux::util::{FeedOption, TimePeriod};
use roux::Subreddit;
use serenity::builder::CreateEmbed;

#[derive(Default)]
pub struct RedditCommand {}

impl RedditCommand {
    pub async fn random_image(&self, subreddit: String) -> Result<CommandResponse, String> {
        let result = Subreddit::new(&subreddit)
            .top(20, Some(FeedOption::new().period(TimePeriod::ThisWeek)))
            .await
            .map_err(|err| err.to_string())?;

        let pos_capy = result
            .data
            .children
            .iter()
            .map(|thing| &thing.data)
            .filter_map(|post| post.url.as_ref())
            .filter(|post| !post.contains("gallery"))
            .choose(&mut thread_rng());

        if let Some(capy) = pos_capy {
            let mut embed = CreateEmbed::default();
            embed.image(capy);
            println!("url: {}", capy);
            Ok(CommandResponse {
                embed: Some(embed),
                ..Default::default()
            })
        } else {
            Ok(CommandResponse {
                ..Default::default()
            })
        }
    }
}
