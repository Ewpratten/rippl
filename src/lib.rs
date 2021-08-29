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
/// let url = process_remote_image("https://www.rust-lang.org/logos/rust-logo-512x512.png", |img| {)
pub async fn process_remote_image(
    url: &url::Url,
    pipeline: fn(DynamicImage) -> Option<DynamicImage>,
) -> Result<url::Url, Error> {
    // Fetch the remote image to a file
    let data = reqwest::get(url.to_string()).await?.bytes().await?;

    // Load the image as something we can manipulate
    let img = ImageReader::new(Cursor::new(data)).decode()?;

    // Operate on the image
    if let Some(img) = pipeline(img) {
        // Upload the image to imgur
        let imgur_handle = imgur::Handle::new("332741bbdcde865".to_string());
        let info = imgur_handle.upload(img.as_bytes())?;

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

#[cfg(test)]
mod tests {
    use super::*;
    use url::Url;

    #[test]
    fn test_process_remote_image() {
        tokio_test::block_on(async {
            let input_url =
                Url::parse("https://www.rust-lang.org/logos/rust-logo-512x512.png").unwrap();
            let output_url = process_remote_image(&input_url, |img| {
                let img = img.clone();
                img.resize(100, 100, image::imageops::FilterType::Nearest);
                Some(img)
            })
            .await;

            // assert!(output_url.is_ok());

            let output_url = output_url.unwrap();
            assert!(output_url.to_string().contains("i.imgur.com"));
            println!("{:?}", output_url);
        })
    }
}
