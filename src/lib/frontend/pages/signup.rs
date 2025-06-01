use sycamore::prelude::*;

use crate::frontend::atoms::{Card, CardContent, CardFooter, CardHeader, CardTitle};
use crate::frontend::organisms::SignupForm;

#[component]
pub fn SignupPage() -> View {
    view! {
        div(id="signup-page") {
            div(id="signup-container") {
                Card {
                    CardHeader {
                        CardTitle(text="Sign up for an account")
                        "Enter your email and create a password"
                    }
                    CardContent {
                        SignupForm()
                    }
                    CardFooter {
                        div(class="text-center") {
                            "Already a user? "
                            a(href="/login") { "Login" }
                        }
                    }
                }
            }
        }
    }
}
