//! This is the fetcher for stuff which varies based on if it's SSR or CSR

use std::{rc::Rc, sync::Arc};

use reqwest::Client;
use yew::prelude::*;

use crate::ObjectCache;

#[derive(Clone)]
pub(crate) struct APIContents {
    pub(crate) client: Arc<Client>,
    pub(crate) cache: Rc<ObjectCache>,
}

impl PartialEq for APIContents {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.client, &other.client)
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct APIProviderProps {
    pub children: Children,
}

#[function_component(LinkdokuAPIProvider)]
pub fn core_client_provider(props: &APIProviderProps) -> Html {
    let client = use_state(|| APIContents {
        client: Arc::new(
            Client::builder()
                .build()
                .expect("Unable to construct client"),
        ),
        cache: Rc::new(ObjectCache::new()),
    });

    html! {
        <ContextProvider<APIContents> context={(*client).clone()}>
            { for props.children.iter() }
        </ContextProvider<APIContents>>
    }
}
