use crate::models::err_msg::ErrorInfo;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use thiserror::Error;

pub type DHResult<T> = Result<T, DHError>;

#[derive(Debug, Error)]
pub enum DHError {
    #[error("{0}")]
    ErrorInfo(#[from] ErrorInfo),
    #[error("{0}")]
    UrlParseError(#[from] url::ParseError),
    #[error("{0}")]
    RequestError(#[from] reqwest::Error),
}

#[derive(Debug, Deserialize, Serialize)]
pub enum ErrorCode {
    InvalidUriSpec,
    ProjectAlreadyExist,
    InvalidParameter,
    InvalidSubscription,
    InvalidCursor,

    ResourceNotFound,
    NoSuchTopic,
    NoSuchProject,
    NoSuchSubscription,
    NoSuchShard,
    NoSuchConnector,
    NoSuchMeterInfo,
    NoSuchAlarmRule,

    ResourceAlreadyExist,
    TopicAlreadyExist,
    ConnectorAlreadyExist,
    UserAlreadyExist,
    AlarmRuleAlreadyExist,

    ResourceLimit,
    SeekOutOfRange,
    Unauthorized,
    NoPermission,
    InvalidShardOperation,
    OperatorDenied,
    LimitExceed,
    InternalServerError,
    SubscriptionOffline,
    OffsetReseted,
    OffsetSessionClosed,
    OffsetSessionChanged,
    MalformedRecord,
    NoSuchConsumer,
    ConsumerGroupInProcess,
    ExpiredAccessToken,
    TopicOffline,
}

impl Display for ErrorCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
