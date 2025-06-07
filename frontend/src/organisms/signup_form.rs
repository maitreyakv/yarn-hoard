use std::sync::Once;

use reqwest::StatusCode;
use secrecy::SecretString;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
use tracing::{debug, error, info};

use crate::atoms::{Button, Toaster};
use crate::molecules::{EmailInput, PasswordInput};
use crate::util::inject_style_tag_into_document_head;
use crate::{ApiClient, ApiClientError};

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
    let status = create_signal(SubmitStatus::None);

    let handle_submit = move |event: SubmitEvent| {
        event.prevent_default();

        if status.get() == SubmitStatus::Pending {
            debug!("Submission already in progress, aborting this submit");
            return;
        }

        if !form.is_valid.get() {
            debug!("Form is not valid, aborting this submit");
            return;
        }

        sycamore::futures::spawn_local({
            let api_client = api_client.clone();
            async move {
                status.set(SubmitStatus::Pending);
                let result = form.submit(api_client).await;
                match result {
                    Ok(_) => {
                        info!("Submit was successful");
                        status.set(SubmitStatus::Success);
                    }
                    Err(api_client_error) => {
                        error!(?api_client_error, "Submit failed!");
                        toaster.warn(match api_client_error {
                            ApiClientError::ErrorStatusCode(StatusCode::CONFLICT) => {
                                format!("That email is already in use, please try logging in.")
                            }
                            _ => {
                                format!(
                                    "There was an issue creating your account, please try again."
                                )
                            }
                        });
                        status.set(SubmitStatus::Failure);
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
    is_valid: ReadSignal<bool>,
}

impl Form {
    fn new() -> Self {
        let email = create_signal(String::new());
        let password = create_signal(String::new());
        let is_valid = create_memo(move || {
            if email.get_clone().is_empty() {
                return false;
            }
            if password.get_clone().len().lt(&8) {
                return false;
            }
            true
        });
        Self {
            email,
            password,
            is_valid,
        }
    }

    #[tracing::instrument(skip_all, err)]
    async fn submit(self, api_client: ApiClient) -> Result<(), ApiClientError> {
        let email = self.email.get_clone();
        let password = SecretString::new(self.password.get_clone().into());
        api_client.create_user(&email, &password).await.map(|_| ())
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SubmitStatus {
    None,
    Pending,
    Success,
    Failure,
}
