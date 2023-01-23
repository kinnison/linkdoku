//! Activities for linkdoku
//!
//! These include things like the login activity as far as databases go

use common::{APIError, BadShortNameReason};

pub mod login;
pub mod puzzle;
pub mod role;

pub enum ActivityError {
    PermissionDenied,
    InvalidInput,
    ShortNameInUse,
    NotFound,
    Error(diesel::result::Error),
    JsonError(serde_json::Error),
    TimeFormatError(time::error::Format),
}

pub type ActivityResult<T> = Result<T, ActivityError>;

impl From<ActivityError> for APIError {
    fn from(value: ActivityError) -> Self {
        match value {
            ActivityError::ShortNameInUse => APIError::BadShortName(BadShortNameReason::NotUnique),
            ActivityError::NotFound => APIError::ObjectNotFound,
            ActivityError::PermissionDenied => APIError::PermissionDenied,
            ActivityError::InvalidInput => APIError::BadInput,
            ActivityError::Error(e) => APIError::DatabaseError(e.to_string()),
            ActivityError::JsonError(e) => APIError::Generic(e.to_string()),
            ActivityError::TimeFormatError(e) => APIError::Generic(e.to_string()),
        }
    }
}

impl From<diesel::result::Error> for ActivityError {
    fn from(value: diesel::result::Error) -> Self {
        ActivityError::Error(value)
    }
}

impl From<serde_json::Error> for ActivityError {
    fn from(value: serde_json::Error) -> Self {
        ActivityError::JsonError(value)
    }
}

impl From<time::error::Format> for ActivityError {
    fn from(value: time::error::Format) -> Self {
        ActivityError::TimeFormatError(value)
    }
}
