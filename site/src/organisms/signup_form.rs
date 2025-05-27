use secrecy::SecretString;
use sycamore::prelude::*;
use sycamore::web::bind::value;
use sycamore::web::events::{SubmitEvent, submit};
use sycamore::web::tags::*;

#[component]
#[tracing::instrument()]
pub fn SignupForm() -> View {
    let api_client = use_context::<api_client::ApiClient>();
    let submit_was_success: Signal<Option<bool>> = create_signal(None);
    let form_ = Form::new();

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        let api_client = api_client.clone();
        sycamore::futures::spawn_local(async move {
            submit_was_success.set(Some(form_.submit(api_client).await.is_ok()));
        });
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
                button()
                    .disabled(move || !form_.is_valid.get())
                    .children("Create Account"),
                move || match submit_was_success.get() {
                    Some(true) => Some("Success!".into()),
                    Some(false) => Some("Failure!".into()),
                    None => None,
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
