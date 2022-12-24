//! Components for toasting
//!

use std::rc::Rc;

use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;

use crate::{Toast, ToastLevel};

use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Serialize, Deserialize)]
struct ToastListEntry {
    nr: usize,
    toast: Toast,
    age: usize,
}

#[derive(PartialEq, Serialize, Deserialize)]
struct ToastList {
    loaded: bool,
    nr: usize,
    paused: bool,
    toasts: Vec<ToastListEntry>,
}

impl ToastList {
    const TICK_TIME_MILLIS: usize = 10;

    fn new() -> Self {
        ToastList {
            loaded: false,
            nr: 0,
            paused: false,
            toasts: vec![],
        }
    }

    fn iter(&self) -> impl Iterator<Item = &ToastListEntry> + DoubleEndedIterator {
        self.toasts.iter()
    }

    fn is_empty(&self) -> bool {
        self.toasts.is_empty()
    }

    fn needs_ticking(&self) -> bool {
        (!self.paused)
            && self
                .toasts
                .iter()
                .any(|entry| entry.toast.lifetime().is_some())
    }

    fn store_to_storage(toasts: &[ToastListEntry]) {
        use gloo::storage::{LocalStorage, Storage};
        LocalStorage::set("toastrack", toasts)
            .expect("Unable to store toastrack into LocalStorage");
    }

    fn load_from_storage() -> Vec<ToastListEntry> {
        use gloo::storage::{LocalStorage, Storage};
        LocalStorage::get("toastrack").unwrap_or_else(|_| Vec::new())
    }
}

enum ToastListAction {
    NewToast(Toast),
    TimerTick,
    Close(usize),
    Pause,
    Unpause,
}

impl Reducible for ToastList {
    type Action = ToastListAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        match action {
            ToastListAction::TimerTick => {
                if !self.loaded {
                    // Load from storage
                    let rack: Vec<ToastListEntry> = Self::load_from_storage();
                    let nr = rack.iter().fold(0, |acc, e| acc.max(e.nr)) + rack.len();
                    Rc::new(Self {
                        loaded: true,
                        nr,
                        paused: false,
                        toasts: rack,
                    })
                } else if self.paused || self.is_empty() {
                    self
                } else {
                    let mut ret = self.toasts.clone();
                    for entry in &mut ret {
                        if entry.toast.lifetime().is_some() {
                            entry.age += Self::TICK_TIME_MILLIS;
                        }
                    }
                    ret.retain(|entry| {
                        if let Some(lifetime) = entry.toast.lifetime() {
                            entry.age <= lifetime
                        } else {
                            true
                        }
                    });

                    Self::store_to_storage(&ret);
                    Rc::new(Self {
                        loaded: true,
                        nr: self.nr,
                        paused: self.paused && !ret.is_empty(),
                        toasts: ret,
                    })
                }
            }
            ToastListAction::NewToast(toast) => {
                let mut ret = self.toasts.clone();
                ret.push(ToastListEntry {
                    nr: self.nr,
                    toast,
                    age: 0,
                });

                Self::store_to_storage(&ret);
                Rc::new(Self {
                    loaded: true,
                    nr: self.nr + 1,
                    paused: self.paused,
                    toasts: ret,
                })
            }
            ToastListAction::Close(nr) => {
                let mut ret = self.toasts.clone();
                ret.retain(|v| v.nr != nr);

                Self::store_to_storage(&ret);
                Rc::new(Self {
                    loaded: true,
                    nr: self.nr,
                    paused: self.paused && !ret.is_empty(),
                    toasts: ret,
                })
            }
            ToastListAction::Pause => Rc::new(Self {
                loaded: true,
                paused: true,
                nr: self.nr,
                toasts: self.toasts.clone(),
            }),
            ToastListAction::Unpause => Rc::new(Self {
                loaded: true,
                paused: false,
                nr: self.nr,
                toasts: self.toasts.clone(),
            }),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ToastLocation {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

impl ToastLocation {
    fn style(self) -> Style {
        match self {
            Self::TopLeft => style! {"top: 0px; left: 0px;"},
            Self::TopRight => style! {"top: 0px; right: 0px;"},
            Self::BottomLeft => style!("bottom: 0px; left: 0px;"),
            Self::BottomRight => style!("bottom: 0px; right: 0px;"),
        }
        .unwrap()
    }

    fn reverse(self) -> bool {
        matches!(self, Self::BottomLeft | Self::BottomRight)
    }
}

#[derive(Clone, PartialEq)]
pub struct Toaster {
    sender: Option<UseReducerDispatcher<ToastList>>,
}

impl Toaster {
    pub fn blank() -> Self {
        Self { sender: None }
    }
    pub fn toast(&self, toast: Toast) {
        if let Some(sender) = &self.sender {
            sender.dispatch(ToastListAction::NewToast(toast));
        }
    }
}

#[derive(Properties, Clone, PartialEq)]
pub struct ToastContainerProps {
    pub position: Option<ToastLocation>,
    pub children: Children,
}

#[styled_component(ToastContainer)]
pub fn toast_container(props: &ToastContainerProps) -> Html {
    let toasts = use_reducer_eq(ToastList::new);

    let toaster = Toaster {
        sender: Some(toasts.dispatcher()),
    };

    use_effect_with_deps(
        |(active, emitter)| {
            use gloo::timers::callback::Interval;
            let emitter = emitter.clone();
            let active = *active;
            let interval = Interval::new(ToastList::TICK_TIME_MILLIS as u32, move || {
                if active {
                    emitter.dispatch(ToastListAction::TimerTick);
                }
            });

            move || drop(interval)
        },
        (toasts.needs_ticking(), toasts.dispatcher()),
    );

    let location = props.position.unwrap_or(ToastLocation::BottomRight);
    let classes = vec![
        Classes::from("toast-container"),
        location.style().into(),
        style!("position: fixed; width: 20vw; margin: 1.5rem;")
            .unwrap()
            .into(),
    ];

    let onclose = Callback::from({
        let toasts = toasts.dispatcher();
        move |nr| toasts.dispatch(ToastListAction::Close(nr))
    });

    let pause_cb = Callback::from({
        let toasts = toasts.dispatcher();
        move |_| toasts.dispatch(ToastListAction::Pause)
    });

    let unpause_cb = Callback::from({
        let toasts = toasts.dispatcher();
        move |_| toasts.dispatch(ToastListAction::Unpause)
    });

    let toastrack = if toasts.is_empty() {
        html! {}
    } else {
        html! {
            <div class={classes}>
                {
                    {
                        let toasts = toasts.iter().map(|entry|
                            html! {
                                <ToastElement
                                    nr={entry.nr}
                                    message={entry.toast.message().to_string()}
                                    level={entry.toast.level()}
                                    onclose={onclose.clone()}
                                    age={entry.age}
                                    lifetime={entry.toast.lifetime()}
                                    onenter={pause_cb.clone()}
                                    onleave={unpause_cb.clone()}
                                />
                            }
                        );
                        if location.reverse() {
                            toasts.rev().collect::<Html>()
                        } else {
                            toasts.collect::<Html>()
                        }
                    }
                }
            </div>
        }
    };

    html! {
        <ContextProvider<Toaster> context={toaster}>
            { for props.children.iter() }
            { toastrack }
        </ContextProvider<Toaster>>
    }
}

#[derive(Properties, PartialEq)]
struct ToastProps {
    message: String,
    level: ToastLevel,
    nr: usize,
    onclose: Callback<usize>,
    age: usize,
    lifetime: Option<usize>,
    onenter: Callback<MouseEvent>,
    onleave: Callback<MouseEvent>,
}

#[function_component(ToastElement)]
fn toast(props: &ToastProps) -> Html {
    let classes = vec![
        Classes::from("notification"),
        props.level.classname().into(),
    ];

    let onclick = Callback::from({
        let cb = props.onclose.clone();
        let nr = props.nr;
        move |_| cb.emit(nr)
    });

    let progress = if let Some(lifetime) = props.lifetime {
        let classes = vec![
            Classes::from("progress"),
            style!("position: absolute; left: 0px; bottom: 0px; width: 20vw; height: 0.25rem;")
                .unwrap()
                .into(),
        ];
        let time_left = lifetime.saturating_sub(props.age);
        let percent = format!("{}%", (time_left * 100) / lifetime);
        let age = format!("{}", time_left);
        let lifetime = format!("{}", lifetime);
        html! {
            <progress class={classes} value={age} max={lifetime}>{percent}</progress>
        }
    } else {
        html! {}
    };

    html! {
        <div class={classes} key={format!("toast-{}", props.nr)} onmouseenter={props.onenter.clone()} onmouseleave={props.onleave.clone()} onmouseover={props.onenter.clone()}>
            <button class="delete" onclick={onclick}></button>
            {props.message.clone()}
            {progress}
        </div>
    }
}
