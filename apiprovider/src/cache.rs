use std::{ops::Deref, rc::Rc};

use async_trait::async_trait;
use bounce::{
    query::{use_query, Query, QueryResult, UseQueryHandle},
    BounceStates,
};
use common::APIError;
use serde::de::DeserializeOwned;
use yew::{prelude::*, suspense::SuspensionResult};

use crate::{use_apiprovider, LinkdokuAPI};

mod seal {
    pub trait Sealed {}
}

pub trait Cacheable: DeserializeOwned + Clone + PartialEq + seal::Sealed {
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
cacheable!(Puzzle, "puzzle");

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct CacheQueryInput {
    uuid: AttrValue,
    api: LinkdokuAPI,
}

#[derive(Clone, PartialEq)]
pub struct QueryCachedValue<T> {
    value: Option<T>,
}

pub type CachedValue<T> = Rc<QueryCachedValue<T>>;

impl<T> Deref for QueryCachedValue<T> {
    type Target = Option<T>;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> QueryCachedValue<T> {
    pub fn get(&self) -> Option<&T> {
        self.value.as_ref()
    }
}

#[async_trait(?Send)]
impl<T> Query for QueryCachedValue<T>
where
    T: Cacheable,
{
    type Input = CacheQueryInput;
    type Error = APIError;

    async fn query(_states: &BounceStates, input: Rc<CacheQueryInput>) -> QueryResult<Self> {
        match input
            .api
            .get_generic_obj::<T>(T::api_name(), &input.uuid)
            .await
        {
            Ok(value) => Ok(QueryCachedValue { value: Some(value) }.into()),
            Err(APIError::ObjectNotFound) => Ok(QueryCachedValue { value: None }.into()),
            Err(e) => Err(e),
        }
    }
}

pub type CacheQueryOutput<T> = UseQueryHandle<QueryCachedValue<T>>;

#[hook]
pub fn use_cached_value<T: Cacheable + 'static>(
    uuid: AttrValue,
) -> SuspensionResult<CacheQueryOutput<T>> {
    let api = use_apiprovider();
    let query_input = use_memo(
        |(api, uuid)| CacheQueryInput {
            api: api.clone(),
            uuid: uuid.clone(),
        },
        (api, uuid),
    );
    use_query::<QueryCachedValue<T>>(query_input)
}
