use std::error::Error;

use qbittorrent::error::ClientError as QClientError;

#[derive(Debug)]
pub enum ClientError {
    /// Http error
    Http(Box<dyn Error + Send + Sync>),

    /// Authorization error
    Authorization,

    /// Parsing error (json for qBittorrent)
    Parsing(Box<dyn Error + Send + Sync>),
}

impl From<QClientError> for ClientError {
    fn from(err: QClientError) -> Self {
        match err {
            QClientError::Http(err) => ClientError::Http(Box::new(err)),
            QClientError::Authorization => ClientError::Authorization,
            QClientError::Json(err) => ClientError::Parsing(Box::new(err)),
        }
    }
}