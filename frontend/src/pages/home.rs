use components::{layout::MainPageLayout, puzzle::PuzzleList};
use frontend_core::component::core::OpenGraphMeta;
use yew::prelude::*;

use crate::util_components::Title;

#[function_component(HomePage)]
pub fn render_home() -> Html {
    html! {
        <MainPageLayout>
            <Title value="Home" />
            <OpenGraphMeta
                description="A sudoku puzzle website"
            />
            <h1 class="title">{"Recently published/updated puzzles"}</h1>
            <hr width="40%" />
            <PuzzleList show_role={true}/>
        </MainPageLayout>
    }
}
