use sycamore::prelude::*;

use crate::organisms::SignupForm;

#[component]
pub fn SignupPage() -> View {
    view! {
        div {
            SignupForm()
            span {
                "Already a user? "
                a(href="/login") { "Login" }
            }
        }
    }
}
