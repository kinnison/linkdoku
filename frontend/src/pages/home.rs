use components::layout::MainPageLayout;
use frontend_core::component::core::OpenGraphMeta;
use yew::prelude::*;
use yew_toastrack::{use_toaster, Toast};

use crate::util_components::Title;

#[function_component(HomePage)]
pub fn render_home() -> Html {
    let toaster = use_toaster();

    let onclick = Callback::from(move |_| {
        toaster.toast(Toast::new("This is a toast message").with_lifetime(2000));
    });

    html! {
        <MainPageLayout>
            <Title value="Home" />
            <OpenGraphMeta
                description="A sudoku puzzle website"
            />
            <button onclick={onclick}>
              { "Click me" }
            </button>
        </MainPageLayout>
    }
}
