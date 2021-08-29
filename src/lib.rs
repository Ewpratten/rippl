//! # Remote Image Processing Pipeline Library
//!
//! This library provides a simple, high-level interface for fetching a remote image,
//! operating on it, then re-publishing it and getting a URL to the new image.

use std::io::Cursor;

use image::io::Reader as ImageReader;
use image::DynamicImage;

pub mod error;
use crate::error::Error;

/// Process a remote image with a pipeline function
///
/// # Example
/// ```
/// # use rippl::process_remote_image;
/// # use url::Url;
/// # tokio_test::block_on(async {
/// let input_url =
/// Url::parse("https://www.rust-lang.org/logos/rust-logo-128x128.png").unwrap();
/// let output_url = process_remote_image(&input_url, |img| {
///     let img = img.clone();
///     img.resize(100, 100, image::imageops::FilterType::Nearest);
///     Some(img)
/// })
/// .await.unwrap();
///
/// assert!(output_url.to_string().contains("i.imgur.com"));
/// # })
/// ```
pub async fn process_remote_image(
    url: &url::Url,
    pipeline: fn(DynamicImage) -> Option<DynamicImage>,
) -> Result<url::Url, Error> {
    // Fetch the remote image to a file
    let data = reqwest::get(url.to_string()).await?.bytes().await?;
    // println!("{:?}", data);

    // Load the image as something we can manipulate
    let img = ImageReader::new(Cursor::new(data)).with_guessed_format()?.decode()?;

    // Operate on the image
    if let Some(img) = pipeline(img) {

        // Save to a bytestream
        let mut out_bytes: Vec<u8> = Vec::new();
        img.write_to(&mut out_bytes, image::ImageOutputFormat::Png)?;

        // Upload the image to imgur
        let imgur_handle = imgur::Handle::new("332741bbdcde865".to_string());
        let info = imgur_handle.upload(&out_bytes)?;

        // Handle the response
        if let Some(url) = info.link() {
            return Ok(url::Url::parse(url)?);
        } else {
            return Err(Error::NoUrlErr);
        }
    } else {
        return Err(Error::PipelineErr);
    }
}