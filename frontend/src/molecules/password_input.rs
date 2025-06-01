use std::sync::Once;

use sycamore::prelude::*;

use crate::util::inject_style_tag_into_document_head;

static INJECT_STYLE: Once = Once::new();

#[component(inline_props)]
pub fn PasswordInput(#[prop(setter(into))] bind: Signal<String>) -> View {
    INJECT_STYLE.call_once(|| {
        inject_style_tag_into_document_head(include_str!("password_input.css"));
    });
    view! {
        div(class="password-input") {
            div {
                label(class="label") { "Password" }
            }
            div {
                input(class="input", r#type="password", bind:value=bind)
            }
        }
    }
}
