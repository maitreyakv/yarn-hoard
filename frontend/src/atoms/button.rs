use std::sync::Once;

use sycamore::prelude::*;

use crate::util::inject_style_tag_into_document_head;

static INJECT_STYLE: Once = Once::new();

#[component(inline_props)]
pub fn Button(
    #[prop(setter(into))] children: Children,
    #[prop(setter(into))] disabled: Option<MaybeDyn<bool>>,
) -> View {
    INJECT_STYLE.call_once(|| {
        inject_style_tag_into_document_head(include_str!("button.css"));
    });
    let disabled = disabled.unwrap_or(MaybeDyn::Static(false));
    view! {
        button(class="button", disabled=disabled) {
            (children)
        }
    }
}
