use yew::prelude::*;

#[derive(Clone, Properties, Default, PartialEq, Eq)]
pub struct AvatarProps {
    pub gravatar_hash: AttrValue,
}

#[function_component(Avatar)]
pub fn user_avatar(props: &AvatarProps) -> Html {
    // Email provided, so try and do a gravatar
    html! {
        <figure class={"image is-32x32 mr-2"}>
            <img class={"is-rounded"} style={"max-height: inherit;"} src={format!("https://www.gravatar.com/avatar/{}?d=robohash", props.gravatar_hash)} />
        </figure>
    }
}
