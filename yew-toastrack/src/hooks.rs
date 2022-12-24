use yew::prelude::*;

use crate::Toaster;

#[hook]
pub fn use_toaster() -> Toaster {
    use_context::<Toaster>().unwrap_or_else(Toaster::blank)
}
