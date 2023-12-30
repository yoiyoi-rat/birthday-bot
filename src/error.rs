use thiserror::Error;


#[derive(Error, Debug)]
pub enum AppError {
    #[error("failed to send request")]
    FailedReplyError(),
    #[error("unknown app error")]
    Unknown,
}