//! This is the fetcher for stuff which varies based on if it's SSR or CSR

use std::sync::Arc;

use reqwest::Client;
use yew::prelude::*;

#[derive(Debug, Clone)]
pub struct ReqwestClient {
    pub(crate) client: Arc<Client>,
}

impl PartialEq for ReqwestClient {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.client, &other.client)
    }
}

#[derive(Clone, Properties, PartialEq)]
pub struct ClientProviderProps {
    pub children: Children,
}

#[function_component(ClientProvider)]
pub fn core_client_provider(props: &ClientProviderProps) -> Html {
    let client = use_state(|| ReqwestClient {
        client: Arc::new(
            Client::builder()
                .build()
                .expect("Unable to construct client"),
        ),
    });

    html! {
        <ContextProvider<ReqwestClient> context={(*client).clone()}>
            { for props.children.iter() }
        </ContextProvider<ReqwestClient>>
    }
}
