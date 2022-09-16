use anyhow::{Context, Result};
use image::codecs::jpeg::JpegEncoder;
use image::imageops::{FilterType, Nearest};
use image::io::Reader;
use serenity::builder::CreateEmbed;
use serenity::model::prelude::AttachmentType;
use std::borrow::Cow;
use std::io::Cursor;

use crate::CommandResponse;

pub async fn nuke<'a>(url: String) -> Result<CommandResponse<'a>> {
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
    img = img
        .resize(200, 200, Nearest)
        .resize(width, height, Nearest)
        .brighten(70)
        .adjust_contrast(100.0);

    let mut jpeg: Cursor<Vec<u8>> = Cursor::new(vec![]);
    JpegEncoder::new_with_quality(&mut jpeg, 1)
        .encode_image(&img)
        .context("Failed to re-encoded image")?;

    Ok(CommandResponse {
        attachment: Some(AttachmentType::Bytes {
            data: Cow::Owned(jpeg.into_inner()),
            filename: "image.jpeg".to_string(),
        }),
        ..Default::default()
    })
}
