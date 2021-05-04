use thiserror::Error;
use warp::reject::Reject;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Tonic Status Error")]
    TonicStatus(#[from] tonic::Status),
    #[error("Tonic Status Error")]
    TonicTransport(#[from] tonic::transport::Error),
    #[error("Prometheus Error")]
    Prometheus(#[from] prometheus::Error),
    #[error("HTTP Error")]
    Http(#[from] warp::http::Error),
}

impl Reject for Error {}
