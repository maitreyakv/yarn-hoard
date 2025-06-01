use std::sync::Once;

use sycamore::prelude::*;

use crate::util::inject_style_tag_into_document_head;

static INJECT_STYLE: Once = Once::new();

#[component(inline_props)]
pub fn EmailInput(#[prop(setter(into))] bind: Signal<String>) -> View {
    INJECT_STYLE.call_once(|| {
        inject_style_tag_into_document_head(include_str!("email_input.css"));
    });
    view! {
        div(class="email-input") {
            div {
                label(class="label") { "Email" }
            }
            div {
                input(class="input", r#type="email", bind:value=bind)
            }
        }
    }
}
