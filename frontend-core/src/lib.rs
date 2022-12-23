//! Core linkdoku frontend components

use std::rc::Rc;

use url::Url;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct BaseURI {
    pub uri: Rc<AttrValue>,
}

#[derive(Clone, Properties, PartialEq)]
pub struct BaseURIProviderProps {
    pub uri: Option<AttrValue>,
    pub children: Children,
}

#[function_component(BaseURIProvider)]
pub fn core_base_uri_provider(props: &BaseURIProviderProps) -> Html {
    let uri = use_memo(
        |uri| match uri {
            Some(uri) => uri.clone(),
            None => {
                let uri = gloo::utils::document()
                    .base_uri()
                    .expect("Could not read document")
                    .expect("Document lacked .baseURI");
                let mut uri = Url::parse(&uri).expect("BaseURI was bad");
                uri.set_path("/");
                uri.set_fragment(None);
                uri.set_query(None);
                uri.to_string().into()
            }
        },
        props.uri.clone(),
    );

    let context = BaseURI { uri };

    html! {
        <ContextProvider<BaseURI> context={context}>
            { for props.children.iter() }
        </ContextProvider<BaseURI>>
    }
}
