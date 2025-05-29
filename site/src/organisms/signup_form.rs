use secrecy::SecretString;
use sycamore::prelude::*;
use sycamore::web::events::SubmitEvent;
use tracing::{debug, error, info};

use crate::atoms::Button;
use crate::molecules::{EmailInput, PasswordInput};

#[component]
#[tracing::instrument()]
pub fn SignupForm() -> View {
    let api_client = use_context::<api_client::ApiClient>();
    let form = Form::new();
    let status = create_signal(SubmitStatus::None);

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let api_client = api_client.clone();

        if status.get() == SubmitStatus::Pending {
            debug!("Submission already in progress, aborting this submit");
            return;
        }

        sycamore::futures::spawn_local(async move {
            status.set(SubmitStatus::Pending);
            let result = form.submit(api_client).await;
            if result.is_ok() {
                info!("Submit was successful");
                status.set(SubmitStatus::Success);
            } else {
                error!("Submit failed!");
                status.set(SubmitStatus::Failure);
            };
        });
    };

    let status_text = move || match status.get() {
        SubmitStatus::None => "Ready!",
        SubmitStatus::Pending => "Pending...",
        SubmitStatus::Success => "Success!",
        SubmitStatus::Failure => "Failure!",
    };

    view! {
        form(on:submit=on_submit) {
            EmailInput(bind=form.email)
            PasswordInput(bind=form.password)
            Button(disabled=move || !form.is_valid.get()) { "Create Account" }
            (status_text)
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
    async fn submit(
        self,
        api_client: api_client::ApiClient,
    ) -> Result<(), api_client::ApiClientError> {
        let email = self.email.get_clone();
        let password = SecretString::new(self.password.get_clone().into());
        api_client.create_user(&email, &password).await
    }
}

#[derive(Clone, Copy, PartialEq)]
enum SubmitStatus {
    None,
    Pending,
    Success,
    Failure,
}
