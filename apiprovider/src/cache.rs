use std::{
    cell::RefCell,
    collections::{hash_map::Entry, HashMap},
    marker::PhantomData,
    rc::Rc,
    sync::atomic::{AtomicUsize, Ordering},
};

use common::APIResult;
use serde::{de::DeserializeOwned, Serialize};
use state::Container;
use tracing::trace;
use yew::{prelude::*, suspense::*};

use crate::{backend::APIContents, use_apiprovider};

mod seal {
    pub trait Sealed {}
}

pub trait Cacheable:
    Serialize + DeserializeOwned + Clone + PartialEq + std::fmt::Debug + seal::Sealed + 'static
{
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
cacheable!(Tag, "tag");
cacheable!(PuzzleMetadata, "puzzle-metadata");

#[hook]
pub fn use_puzzle_lookup(
    role: AttrValue,
    puzzle: AttrValue,
) -> SuspensionResult<common::APIResult<String>> {
    let api = use_apiprovider();
    let res = use_future_with_deps(
        move |deps| async move { api.lookup_puzzle(deps.0.as_str(), deps.1.as_str()).await },
        (role, puzzle),
    )?;

    Ok((*res).clone().map(|r| r.uuid))
}

#[derive(Clone)]
pub struct Cached<T: Cacheable> {
    cache: Rc<ObjectCache>,
    value: Rc<APIResult<T>>,
}

impl<T: Cacheable> PartialEq for Cached<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.value, &other.value)
    }
}

impl<T: Cacheable> Cached<T> {
    pub fn refresh(&self, uuid: &str, new_value: T) {
        self.cache.insert(uuid, Rc::new(Ok(new_value)))
    }
}

impl<T: Cacheable> AsRef<APIResult<T>> for Cached<T> {
    fn as_ref(&self) -> &APIResult<T> {
        &self.value
    }
}

pub struct ObjectCache {
    next_listener: AtomicUsize,
    content: Container!(),
}

type CacheMap<T> = RefCell<HashMap<String, Rc<APIResult<T>>>>;
type CallbackMap<T> = RefCell<HashMap<String, HashMap<usize, Callback<Rc<APIResult<T>>>>>>;

impl ObjectCache {
    pub(crate) fn new() -> Self {
        Self {
            next_listener: Default::default(),
            content: Default::default(),
        }
    }

    fn cache_map<T: Cacheable>(&self) -> &CacheMap<T> {
        if let Some(value) = self.content.try_get() {
            value
        } else {
            self.content.set(<CacheMap<T>>::default());
            self.content.get()
        }
    }

    fn callback_map<T: Cacheable>(&self) -> &CallbackMap<T> {
        if let Some(value) = self.content.try_get() {
            value
        } else {
            self.content.set(<CallbackMap<T>>::default());
            self.content.get()
        }
    }
    pub fn insert<T: Cacheable>(&self, uuid: &str, value: Rc<APIResult<T>>) {
        match self.cache_map().borrow_mut().entry(uuid.to_string()) {
            Entry::Occupied(mut o) => {
                if o.get() == &value {
                    trace!("Skipped setting {} {}", T::api_name(), uuid);
                    return;
                }
                o.insert(value.clone());
            }
            Entry::Vacant(v) => {
                v.insert(value.clone());
            }
        };
        let cbs = self.callback_map().borrow().get(uuid).cloned();
        for (_, cb) in cbs.iter().flatten() {
            cb.emit(value.clone())
        }
    }

    pub fn get<T: Cacheable>(&self, uuid: &str) -> Option<Rc<APIResult<T>>> {
        self.cache_map().borrow().get(uuid).cloned()
    }

    fn listen<T: Cacheable>(
        self: Rc<Self>,
        uuid: &str,
        cb: Callback<Rc<APIResult<T>>>,
    ) -> ObjectCacheListener<T> {
        let counter = self.next_listener.fetch_add(1, Ordering::SeqCst);
        self.callback_map()
            .borrow_mut()
            .entry(uuid.to_string())
            .or_default()
            .insert(counter, cb);
        ObjectCacheListener {
            cache: self,
            uuid: uuid.into(),
            entry: counter,
            kind: PhantomData,
        }
    }

    fn unlisten<T: Cacheable>(&self, uuid: &str, entry: usize) {
        let mut cbmap = self.callback_map::<T>().borrow_mut();
        cbmap.get_mut(uuid).and_then(|map| map.remove(&entry));
    }
}

struct ObjectCacheListener<T: Cacheable> {
    cache: Rc<ObjectCache>,
    uuid: String,
    entry: usize,
    kind: PhantomData<T>,
}

impl<T: Cacheable> Drop for ObjectCacheListener<T> {
    fn drop(&mut self) {
        self.cache.unlisten::<T>(&self.uuid, self.entry);
    }
}

#[hook]
pub fn use_cache_controller() -> Rc<ObjectCache> {
    let api_content = use_context::<APIContents>().unwrap();
    api_content.cache
}

#[derive(Copy, Clone)]
pub enum CacheLookupKind {
    ByUUID,
    ByShortName,
}

#[hook]
pub fn use_cached_value_<T: Cacheable + 'static>(
    key: AttrValue,
    kind: CacheLookupKind,
) -> SuspensionResult<Cached<T>> {
    let controller = use_cache_controller();
    let api = use_apiprovider();
    #[cfg(feature = "ssr")]
    let pre_api = api.clone();
    let pre_cached = {
        let key = key.to_string();
        use_prepared_state!(
            async move |deps| -> String {
                trace!("Acquiring {} {}", T::api_name(), deps);
                let result = match kind {
                    CacheLookupKind::ByUUID => {
                        pre_api.get_generic_obj::<T>(T::api_name(), &deps).await
                    }
                    CacheLookupKind::ByShortName => {
                        pre_api
                            .get_generic_obj_by_name::<T>(T::api_name(), &deps)
                            .await
                    }
                };
                serde_json::to_string(&result).expect("Woah, can't encode generic object?")
            },
            key
        )
    }?;
    let pre_cache_available = use_state_eq(|| pre_cached.is_some());

    if *pre_cache_available {
        if let Some(pre_cached) = pre_cached {
            trace!("Precaching {} {}", T::api_name(), key);
            let obj: APIResult<T> =
                serde_json::from_str(&pre_cached).expect("Woah, can't decode generic object?");
            controller.insert(&key, Rc::new(obj));
            pre_cache_available.set(false);
        }
    }

    let retrieved = use_state_eq(|| None);

    let listener = use_callback(
        {
            let setter = retrieved.setter();
            let cache = controller.clone();
            let key = key.clone();
            move |value, _| {
                trace!(
                    "Received cache callback for a {} of key {}",
                    T::api_name(),
                    key
                );
                setter.set(Some(Cached {
                    cache: cache.clone(),
                    value,
                }))
            }
        },
        key.clone(),
    );

    use_memo(
        {
            let controller = controller.clone();
            let key = key.clone();
            move |listener: &Callback<Rc<APIResult<T>>>| controller.listen(&key, listener.clone())
        },
        listener,
    );

    use_future_with_deps(
        {
            let setter = retrieved.setter();
            let cache = controller.clone();
            move |key: Rc<AttrValue>| async move {
                // We need to retrieve the relevant type from the API
                if let Some(value) = cache.get(&key) {
                    trace!("Already cached: {} {}", T::api_name(), key);
                    setter.set(Some(Cached {
                        cache: cache.clone(),
                        value,
                    }));
                } else {
                    let result = match kind {
                        CacheLookupKind::ByUUID => {
                            api.get_generic_obj::<T>(T::api_name(), &key).await
                        }
                        CacheLookupKind::ByShortName => {
                            api.get_generic_obj_by_name::<T>(T::api_name(), &key).await
                        }
                    };
                    controller.insert(&key, Rc::new(result));
                }
            }
        },
        key,
    )?;

    retrieved
        .as_ref()
        .cloned()
        .ok_or_else(|| Suspension::new().0)
}

#[hook]
pub fn use_cached_value<T: Cacheable + 'static>(uuid: AttrValue) -> SuspensionResult<Cached<T>> {
    use_cached_value_(uuid, CacheLookupKind::ByUUID)
}

#[hook]
pub fn use_cached_value_by_name<T: Cacheable + 'static>(
    name: AttrValue,
) -> SuspensionResult<Cached<T>> {
    use_cached_value_(name, CacheLookupKind::ByShortName)
}
