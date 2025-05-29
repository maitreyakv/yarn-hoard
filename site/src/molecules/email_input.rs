use sycamore::prelude::*;

#[component(inline_props)]
pub fn EmailInput(#[prop(setter(into))] bind: Signal<String>) -> View {
    view! {
        div {
            div {
                label { "Email" }
            }
            div {
                input(class="input", r#type="email", bind:value=bind)
            }
        }
    }
}
