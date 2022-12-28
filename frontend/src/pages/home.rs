use components::layout::MainPageLayout;
use yew::prelude::*;
use yew_toastrack::{use_toaster, Toast};

#[function_component(HomePage)]
pub fn render_home() -> Html {
    let toaster = use_toaster();

    let onclick = Callback::from(move |_| {
        toaster.toast(Toast::new("This is a toast message").with_lifetime(2000));
    });

    html! {
        <MainPageLayout>
            <button onclick={onclick}>
              { "Click me" }
            </button>
        </MainPageLayout>
    }
}
