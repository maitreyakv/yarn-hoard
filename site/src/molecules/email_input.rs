use sycamore::prelude::*;

#[component(inline_props)]
pub fn EmailInput(#[prop(setter(into))] bind: Signal<String>) -> View {
    view! {
        div {
            label { "Email" }
            input(class="input", r#type="email", bind:value=bind)
        }
    }
}
