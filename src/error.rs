use thiserror::Error;
use awc::error::SendRequestError;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("failed to send request")]
    FailedReplyError(#[from] SendRequestError),
    #[error("json deserialize error")]
    FailedDeserializeError(#[from] serde_json::Error),
    #[error("received unsupported message")]
    UnsupportedMessage,
    #[error("failed to get replytoken")]
    FailedToGetReplyToken,
    #[error("failed to verify signature")]
    FailedToVerifySignature,
}