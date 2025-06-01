use sycamore::prelude::*;

#[component(inline_props)]
pub fn EmailInput(#[prop(setter(into))] bind: Signal<String>) -> View {
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
