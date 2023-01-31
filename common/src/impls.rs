//! Implementations to enable IntoResponse and friends

use axum::{
    response::{IntoResponse, Response},
    Json,
};

use crate::{APIError, APIOutcome};

impl IntoResponse for APIError {
    fn into_response(self) -> Response {
        Json::from(APIOutcome::<()>::Error(self)).into_response()
    }
}

macro_rules! into_resp {
    ($ty:path) => {
        impl IntoResponse for $ty {
            fn into_response(self) -> Response {
                Json::from(APIOutcome::Success(self)).into_response()
            }
        }
    };
}

into_resp!(crate::objects::Puzzle);
into_resp!(crate::objects::Role);
into_resp!(crate::objects::Tag);
into_resp!(crate::objects::PuzzleMetadata);

into_resp!(crate::internal::login::begin::Response);
into_resp!(crate::internal::login::complete::Response);
into_resp!(crate::internal::login::providers::Response);
into_resp!(crate::internal::logout::Response);
into_resp!(crate::internal::util::expand_tinyurl::Response);

into_resp!(crate::public::puzzle::lookup::Response);
into_resp!(crate::public::role::puzzles::Response);
into_resp!(crate::public::tag::list::Response);
into_resp!(crate::public::userinfo::Response);
into_resp!(crate::public::scaffold::Response);
