use sycamore::prelude::*;

#[component(inline_props)]
pub fn PasswordInput(#[prop(setter(into))] bind: Signal<String>) -> View {
    view! {
        div {
            div {
                label { "Password" }
            }
            div {
                input(class="input", r#type="password", bind:value=bind)
            }
        }
    }
}
