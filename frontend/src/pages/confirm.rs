use sycamore::prelude::*;

use crate::atoms::{Card, CardContent, CardFooter, CardHeader, CardTitle};

#[component(inline_props)]
pub fn ConfirmPage(token: Option<String>) -> View {
    view! {
        div(id="confirm-page") {
            div(id="confirm-container") {
                Card {
                    CardHeader {
                        CardTitle(text="Thank you for signing up!")
                    }
                    CardContent {
                        "Please check your email for a confirmation link. "
                        "You may need to check your spam folder."
                    }
                    CardFooter {
                        div(class="text-center") {
                            "Ready to log in? "
                            a(href="/login") { "Login" }
                        }
                    }
                }
            }
        }
    }
}
