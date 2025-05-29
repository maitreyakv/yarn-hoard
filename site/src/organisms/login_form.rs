use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;

use crate::molecules::{EmailInput, PasswordInput};

#[component]
pub fn LoginForm() -> View {
    let email = create_signal(String::new());
    let password = create_signal(String::new());

    let handle_submit = move |event: SubmitEvent| {
        event.prevent_default();
        console_log!("email={email}, password={password}");
    };

    view! {
        form(on:submit=handle_submit) {
            EmailInput(bind=email)
            PasswordInput(bind=password)
        }
    }
}
