use std::sync::Once;

use sycamore::prelude::*;

static INJECT_STYLE_TAG: Once = Once::new();

#[component(inline_props)]
pub fn Button(
    #[prop(setter(into))] children: Children,
    #[prop(setter(into))] disabled: Option<MaybeDyn<bool>>,
) -> View {
    inject_style_tag();
    let disabled = disabled.unwrap_or(MaybeDyn::Static(false));
    view! {
        button(class="button", disabled=disabled) {
            (children)
        }
    }
}

fn inject_style_tag() {
    INJECT_STYLE_TAG.call_once(|| {
        let style_element = document().create_element("style").unwrap();
        style_element.set_inner_html(include_str!("button.css"));
        document()
            .head()
            .unwrap()
            .append_child(&style_element)
            .unwrap();
    });
}
