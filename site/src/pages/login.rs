use sycamore::prelude::*;
use sycamore::web::tags::*;

use crate::organisms::LoginForm;

#[component]
pub fn LoginPage() -> View {
    div()
        .children((LoginForm(), a().href("/signup").children("New user?")))
        .into()
}
