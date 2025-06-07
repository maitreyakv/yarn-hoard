use std::sync::Once;

use sycamore::prelude::*;

use crate::util::inject_style_tag_into_document_head;

static INJECT_STYLE: Once = Once::new();

pub enum Icon {
    Warning,
}

impl Icon {
    pub fn view(&self) -> View {
        view! {
            IconView(path=match self {
                Self::Warning => {
                    "/assets/icons/warning.svg".to_string()
                }
            })
        }
    }
}

#[component(inline_props)]
fn IconView(path: String) -> View {
    INJECT_STYLE.call_once(|| {
        inject_style_tag_into_document_head(include_str!("icon.css"));
    });

    view! {
        img(src=path, class="icon")
    }
}
