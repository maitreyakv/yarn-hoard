use secrecy::SecretString;
use sycamore::prelude::*;
use sycamore::web::bind::value;
use sycamore::web::events::{SubmitEvent, submit};
use sycamore::web::tags::*;
use tracing::{debug, error, info};

use crate::atoms::{Button, Button_Props};

#[component]
#[tracing::instrument()]
pub fn SignupForm() -> View {
    let api_client = use_context::<api_client::ApiClient>();
    let form_ = Form::new();
    let status = create_signal(SubmitStatus::None);

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let api_client = api_client.clone();

        if status.get() == SubmitStatus::Pending {
            debug!("Submission already in progress, aborting this submit");
            return;
        }

        status.set(SubmitStatus::Pending);
        sycamore::futures::spawn_local(async move {
            status.set(if form_.submit(api_client).await.is_ok() {
                info!("Submit was successful");
                SubmitStatus::Success
            } else {
                error!("Submit failed!");
                SubmitStatus::Failure
            });
        });
    };

    let button_text = move || {
        if let SubmitStatus::Pending = status.get() {
            view! { "..." }
        } else {
            view! { "Create Account" }
        }
    };

    form()
        .on(submit, on_submit)
        .children((
            div().children((
                label().children("Email"),
                input().r#type("email").bind(value, form_.email),
            )),
            div().children((
                label().children("Password"),
                input().r#type("password").bind(value, form_.password),
            )),
            div().children((
                move || {
                    Button(
                        Button_Props::builder()
                            .children(button_text)
                            .disabled(move || !form_.is_valid.get())
                            .build(),
                    )
                },
                move || match status.get() {
                    SubmitStatus::None => None,
                    SubmitStatus::Pending => None,
                    SubmitStatus::Success => Some("Success!".into()),
                    SubmitStatus::Failure => Some("Failure!".into()),
                },
            )),
        ))
        .into()
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
