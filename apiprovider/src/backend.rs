//! This is the fetcher for stuff which varies based on if it's SSR or CSR

use std::sync::Arc;

use reqwest::Client;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub(crate) struct APIContents {
    pub(crate) client: Arc<Client>,
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
    });

    html! {
        <ContextProvider<APIContents> context={(*client).clone()}>
            { for props.children.iter() }
        </ContextProvider<APIContents>>
    }
}
