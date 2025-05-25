use sycamore::prelude::*;
use sycamore::web::bind::value;
use sycamore::web::events::{SubmitEvent, submit};
use sycamore::web::tags::*;

#[component]
pub fn SignupForm() -> View {
    let email = create_signal(String::new());
    let password = create_signal(String::new());
    let confirmed_password = create_signal(String::new());

    let on_submit = move |event: SubmitEvent| {
        event.prevent_default();
        console_log!("email={email}, password={password}, confirmed_password={confirmed_password}");
    };

    form()
        .on(submit, on_submit)
        .children((
            div().children(
                input()
                    .r#type("email")
                    .placeholder("Email")
                    .bind(value, email),
            ),
            div().children(
                input()
                    .r#type("password")
                    .placeholder("Password")
                    .bind(value, password),
            ),
            div().children(
                input()
                    .r#type("password")
                    .placeholder("Confirm Password")
                    .bind(value, confirmed_password),
            ),
            div().children(button().children("Create Account")),
        ))
        .into()
}
