use sycamore::prelude::*;

use crate::organisms::LoginForm;

#[component]
pub fn LoginPage() -> View {
    view! {
        div {
            LoginForm()
            span {
                "Don't have an account? "
                a(href="/signup") { "Sign up" }
            }
        }
    }
}
