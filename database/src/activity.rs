//! Activities for linkdoku
//!
//! These include things like the login activity as far as databases go

use common::APIError;

pub mod login;
pub mod role;

pub enum ActivityError {
    PermissionDenied,
    InvalidInput,
    Error(diesel::result::Error),
}

pub type ActivityResult<T> = Result<T, ActivityError>;

impl From<ActivityError> for APIError {
    fn from(value: ActivityError) -> Self {
        match value {
            ActivityError::PermissionDenied => APIError::PermissionDenied,
            ActivityError::InvalidInput => APIError::BadInput,
            ActivityError::Error(e) => APIError::DatabaseError(e.to_string()),
        }
    }
}

impl From<diesel::result::Error> for ActivityError {
    fn from(value: diesel::result::Error) -> Self {
        ActivityError::Error(value)
    }
}
