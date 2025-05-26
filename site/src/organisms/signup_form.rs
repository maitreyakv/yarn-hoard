use std::collections::HashSet;

use sycamore::prelude::*;
use sycamore::web::bind::value;
use sycamore::web::events::{SubmitEvent, submit};
use sycamore::web::tags::*;

#[component]
pub fn SignupForm() -> View {
    let fields = Fields::new();

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        console_log!("{fields:?}");
        // TODO: post new user and redirect to "check email" page

        let email = fields.email.get_clone();
        let password = fields.password.get_clone();

        let future = async move {
            let response = api_client::ApiClient::insecure(std::env!("API_URL"))
                .create_user(&email, &password)
                .await
                .unwrap();
            console_log!("{response:?}")
        };

        sycamore::futures::spawn_local(future);
    };

    form()
        .on(submit, on_submit)
        .children((
            div().children((
                label().children("Email"),
                input().r#type("email").bind(value, fields.email),
            )),
            div().children((
                label().children("Password"),
                input().r#type("password").bind(value, fields.password),
            )),
            div().children(
                button()
                    .disabled(move || fields.validate().is_err())
                    .children("Create Account"),
            ),
        ))
        .into()
}

#[derive(Debug, Clone, Copy)]
struct Fields {
    email: Signal<String>,
    password: Signal<String>,
}

impl Fields {
    fn new() -> Self {
        Self {
            email: create_signal(String::new()),
            password: create_signal(String::new()),
        }
    }

    fn validate(&self) -> Result<(String, String), HashSet<FieldError>> {
        let email = self.email.get_clone();
        let password = self.password.get_clone();

        let mut errors = HashSet::new();
        if email.is_empty() {
            errors.insert(FieldError::NoEmail);
        }
        if password.len().lt(&8) {
            errors.insert(FieldError::ShortPassword);
        }

        if errors.is_empty() {
            Ok((email, password))
        } else {
            Err(errors)
        }
    }
}

#[derive(Eq, Hash, PartialEq)]
enum FieldError {
    NoEmail,
    ShortPassword,
}
