use crate::Ctx;
use anyhow::{Context, Result};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::FilterType;
use image::io::Reader;
use image::{imageops, ImageFormat};
use poise::serenity_prelude::{AttachmentType, Member};
use std::borrow::Cow;
use std::io::Cursor;
use tracing::{info, instrument};

const COPE_IMG: &[u8] = include_bytes!("../../cope.png");

/// Someone coping hard? Call them out on it.
#[poise::command(slash_command)]
#[instrument(skip(ctx), name = "cope")]
pub async fn cope(ctx: Ctx<'_>, #[description = "Whomst be coping"] copee: Member) -> Result<()> {
    ctx.defer().await?;

    let url = copee.avatar.or_else(|| copee.user.avatar_url()).unwrap_or(
        "https://www.meme-arsenal.com/memes/a3da25d288fca9f005449231f4b3927d.jpg".to_string(),
    );
    let mut img = Reader::new(Cursor::new(
        reqwest::get(&url)
            .await
            .context("Failed to fetch avatar")?
            .bytes()
            .await
            .context("Failed to get image data")?,
    ))
    .with_guessed_format()
    .context("Wat, that's a weird image type")?
    .decode()
    .context("Wat, that is some weird image bruv")?;

    let cope = image::load_from_memory_with_format(COPE_IMG, ImageFormat::Png)
        .context("How the fuck can I not load a static image?")?
        .resize_exact(img.width(), img.height(), FilterType::Nearest);
    imageops::overlay(&mut img, &cope, 0, 0);

    let mut jpeg: Cursor<Vec<u8>> = Cursor::new(vec![]);
    JpegEncoder::new_with_quality(&mut jpeg, 90)
        .encode_image(&img)
        .context("Failed to re-encoded image")?;

    ctx.send(|f| {
        f.attachment(AttachmentType::Bytes {
            data: Cow::Owned(jpeg.into_inner()),
            filename: "cope.jpeg".to_string(),
        })
        .content(format!(
            "Cope harder {}",
            copee.nick.unwrap_or(copee.user.name)
        ))
    })
    .await?;

    Ok(())
}
