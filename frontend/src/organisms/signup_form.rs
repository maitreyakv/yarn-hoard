use std::str::FromStr;
use std::sync::Once;

use anyhow::anyhow;
use reqwest::StatusCode;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
use tracing::{debug, error, info, warn};

use crate::atoms::{Button, Toaster};
use crate::molecules::{EmailInput, PasswordInput};
use crate::util::inject_style_tag_into_document_head;
use crate::{ApiClient, ApiClientError, Password};

static INJECT_STYLE: Once = Once::new();

#[component]
#[tracing::instrument()]
pub fn SignupForm() -> View {
    INJECT_STYLE.call_once(|| {
        inject_style_tag_into_document_head(include_str!("signup_form.css"));
    });

    let toaster = use_context::<Toaster>();
    let api_client = use_context::<ApiClient>();
    let form = Form::new();
    let status = create_signal(Status::None);

    let handle_submit = move |event: SubmitEvent| {
        event.prevent_default();

        if let Status::Pending = status.get() {
            debug!("Submission already in progress, aborting this submit");
            return;
        }

        sycamore::futures::spawn_local({
            let api_client = api_client.clone();
            async move {
                status.set(Status::Pending);
                let result = form.submit(api_client).await;
                match result {
                    Ok(_) => {
                        info!("Submit was successful");
                        status.set(Status::Success);
                        window()
                            .location()
                            .set_href("/confirm")
                            .unwrap_or_else(|_| error!("Failed to navigate to `/confirm`!"));
                    }
                    Err(error) => {
                        warn!("{error}");
                        toaster.warn(error.to_string());
                        status.set(Status::Failure);
                    }
                };
            }
        });
    };

    view! {
        form(class="signup-form", on:submit=handle_submit) {
            EmailInput(bind=form.email)
            PasswordInput(bind=form.password)
            Button { "Create Account" }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Form {
    email: Signal<String>,
    password: Signal<String>,
}

impl Form {
    fn new() -> Self {
        let email = create_signal(String::new());
        let password = create_signal(String::new());
        Self { email, password }
    }

    #[tracing::instrument]
    async fn submit(self, api_client: ApiClient) -> anyhow::Result<()> {
        let email = email_address::EmailAddress::from_str(&self.email.get_clone())
            .map_err(|error| anyhow!("Email is invalid! {error}"))?;
        let password = Password::try_from(self.password.get_clone())?;
        Ok(api_client
            .create_user(&email, &password)
            .await
            .map(|_| ())
            .map_err(|error| match error {
                ApiClientError::ErrorStatusCode(StatusCode::CONFLICT) => {
                    anyhow!("That email is already in use, please try logging in!")
                }
                _ => {
                    anyhow!("There was an issue creating your account, please try again.")
                }
            })?)
    }
}

#[derive(Clone, Copy)]
enum Status {
    None,
    Pending,
    Success,
    Failure,
}
