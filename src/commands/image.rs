use crate::Ctx;

use anyhow::{Context, Result};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::{FilterType, Nearest};
use image::io::Reader;
use mime::Mime;
use poise::futures_util::StreamExt;
use poise::serenity_prelude::{Attachment, AttachmentType, CacheHttp, Embed, Message};
use poise::{serenity_prelude as serenity, CreateReply};
use std::borrow::Cow;
use std::future;
use std::io::Cursor;
use std::str::FromStr;
use tracing::{debug, instrument};

/// Nuke the previous image
#[poise::command(slash_command)]
#[instrument(skip(ctx), name = "nuke")]
pub async fn nuke(ctx: Ctx<'_>) -> Result<()> {
    ctx.defer().await?;
    let msg_res = Box::pin(
        ctx.channel_id()
            .messages_iter(&ctx.http())
            .filter(|res| future::ready(res.is_ok()))
            .map(|res| res.unwrap())
            .take(20)
            .filter(|msg: &Message| {
                future::ready(
                    msg.attachments.iter().any(|a: &Attachment| {
                        a.content_type
                            .as_ref()
                            .filter(|c| filter_image_mine(c))
                            .is_some()
                    }) || msg.embeds.iter().any(|embed: &Embed| embed.image.is_some()),
                )
            }),
    )
    .next()
    .await;

    let url: String = if let Some(msg) = msg_res {
        debug!(
            message_author = msg.user.name,
            message_id = msg.id,
            "image message found"
        );
        msg.attachments
            .iter()
            .filter(|a| {
                a.content_type
                    .as_ref()
                    .filter(|c| filter_image_mine(c))
                    .is_some()
            })
            .map(|a| &a.url)
            .next()
            .or_else(|| {
                msg.embeds
                    .iter()
                    .filter_map(|e: &Embed| e.image.as_ref())
                    .map(|ei| &ei.url)
                    .next()
            })
            .expect(
                "Stuart assumed the message filter would ensure the message would have an image",
            )
            .to_string()
    } else {
        return ctx
            .say("Bruv, looked 20 messages back, no fucking images")
            .await
            .map(|_| ())
            .context("Failed to send reply");
    };
    debug!(url);

    let mut img = Reader::new(Cursor::new(
        reqwest::get(url)
            .await
            .context("Failed to fetch image")?
            .bytes()
            .await
            .context("Failed to get image data")?,
    ))
    .with_guessed_format()
    .context("Wat, that is some weird image type")?
    .decode()
    .context("Wat, that is some weird image bruv")?;

    let width = img.width();
    let height = img.height();
    debug!(old_width = width, old_height = height);
    img = img
        .resize(200, 200, Nearest)
        .resize(width, height, Nearest)
        .brighten(70)
        .adjust_contrast(100.0);

    let mut jpeg: Cursor<Vec<u8>> = Cursor::new(vec![]);
    JpegEncoder::new_with_quality(&mut jpeg, 1)
        .encode_image(&img)
        .context("Failed to re-encoded image")?;
    debug!(final_img_size = jpeg.get_ref().len());

    ctx.send(|f: &mut CreateReply| {
        f.attachment(AttachmentType::Bytes {
            data: Cow::Owned(jpeg.into_inner()),
            filename: "image.jpeg".to_string(),
        })
        .reply(true)
    })
    .await?;

    Ok(())
}

fn filter_image_mine(m: &str) -> bool {
    if let Ok(mime_val) = Mime::from_str(m) {
        mime_val.type_() == mime::IMAGE
    } else {
        false
    }
}
