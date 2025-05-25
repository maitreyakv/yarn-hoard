use sycamore::prelude::*;
use sycamore::web::tags::*;

use crate::organisms::SignupForm;

#[component]
pub fn SignupPage() -> View {
    div()
        .children((SignupForm(), a().href("/login").children("Already a user?")))
        .into()
}
