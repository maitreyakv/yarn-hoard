use sycamore::prelude::*;
use sycamore::web::tags::*;

#[component]
pub fn LandingPage() -> View {
    div()
        .children((
            div().children("Hello World!"),
            div().children(a().href("/login").children("Login")),
            div().children(a().href("/signup").children("Signup")),
        ))
        .into()
}
