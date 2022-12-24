use common::APIError;
use serde::de::DeserializeOwned;
use yew::{
    prelude::*,
    suspense::{use_future, SuspensionResult},
};

use crate::use_apiprovider;

#[derive(Clone)]
pub enum CachedValue<T: Clone> {
    Missing,
    Error(APIError),
    Value(T),
}

impl<T: Clone> CachedValue<T> {
    pub fn unwrap(self) -> T {
        match self {
            Self::Missing => panic!("Attempt to unwrap a missing cached value"),
            Self::Error(e) => panic!("Attempt to unwrap an errored cached value: {e:?}"),
            Self::Value(v) => v,
        }
    }
}

mod seal {
    pub trait Sealed {}
}

pub trait Cacheable: DeserializeOwned + Clone + seal::Sealed {
    fn api_name() -> &'static str;
}

macro_rules! cacheable {
    ($obj:ident, $name:literal) => {
        impl seal::Sealed for common::objects::$obj {}
        impl Cacheable for common::objects::$obj {
            fn api_name() -> &'static str {
                $name
            }
        }
    };
}

cacheable!(Role, "role");

#[hook]
pub fn use_cached_value<T: Cacheable + 'static>(
    uuid: AttrValue,
) -> SuspensionResult<CachedValue<T>> {
    let api = use_apiprovider();
    let fetched = use_future(|| async move {
        match api.get_generic_obj::<T>(T::api_name(), &uuid).await {
            Ok(obj) => CachedValue::Value(obj),
            Err(APIError::ObjectNotFound) => CachedValue::Missing,
            Err(e) => CachedValue::Error(e),
        }
    })?;

    Ok((*fetched).clone())
}
