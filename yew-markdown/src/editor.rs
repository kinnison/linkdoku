//! Editor for markdown - this is a tabbed control using Bulma CSS
//! classes to control the tabs etc.  We show a text area of the name
//! provided to us, along with a preview pane.
//!
//! For now, text is fixed, but later we will support changing that too

use web_sys::HtmlTextAreaElement;
use yew::prelude::*;
use yew_bulma_tabs::{TabContent, Tabbed};

use crate::{render::MarkdownRender, xform::Transformer};

#[derive(Properties, PartialEq)]
pub struct MarkdownEditorProps {
    pub initial: AttrValue,
    pub onchange: Option<Callback<AttrValue>>,
    pub transformer: Option<Transformer>,
    pub help: Option<AttrValue>,
}

#[function_component(MarkdownEditor)]
pub fn markdown_editor(props: &MarkdownEditorProps) -> Html {
    let markdown = use_state(|| props.initial.clone());
    let changed = use_state(|| false);

    let editor = use_node_ref();

    if markdown.as_ref() != props.initial.as_ref() && !*changed {
        markdown.set(props.initial.clone());
    }

    let onchange = {
        let setter = markdown.clone();
        let editor = editor.clone();
        let parent_onchange = props.onchange.clone();
        let changed_setter = changed.setter();
        Callback::from(move |_| {
            let editor: HtmlTextAreaElement = editor.cast().unwrap();
            let value: AttrValue = editor.value().into();
            changed_setter.set(true);
            if let Some(cb) = &parent_onchange {
                cb.emit(value.clone());
            }
            setter.set(value);
        })
    };

    let oninput = {
        let setter = markdown.clone();
        let editor = editor.clone();
        let parent_onchange = props.onchange.clone();
        let changed_setter = changed.setter();
        Callback::from(move |_| {
            let editor: HtmlTextAreaElement = editor.cast().unwrap();
            let value: AttrValue = editor.value().into();
            changed_setter.set(true);
            if let Some(cb) = &parent_onchange {
                cb.emit(value.clone());
            }
            setter.set(value);
        })
    };

    let help_tab = props.help.as_ref().map(|help| {
        html_nested! {
            <TabContent title={"Markdown Help"}>
                <MarkdownRender markdown={help.clone()} />
            </TabContent>
        }
    });

    html! {
        <Tabbed default={"Write"}>
            <TabContent title={"Write"}>
                <textarea ref={editor} onchange={onchange} oninput={oninput} class={"textarea is-family-code"} value={(*markdown).clone()} />
            </TabContent>
            {help_tab}
            <TabContent title={"Preview"}>
                <MarkdownRender markdown={(*markdown).clone()} transformer={props.transformer.clone()} />
            </TabContent>
        </Tabbed>
    }
}
