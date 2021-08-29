use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    ReqwestErr(#[from] reqwest::Error),

    #[error(transparent)]
    ImageErr(#[from] image::ImageError),

    #[error(transparent)]
    IoErr(#[from] std::io::Error),

    #[error(transparent)]
    ImgurErr(#[from] imgur2018::Error),

    #[error(transparent)]
    UrlErr(#[from] url::ParseError),

    #[error("Pipeline failed to operate on image data")]
    PipelineErr,

    #[error("Imgur did not return a URL")]
    NoUrlErr,
}
