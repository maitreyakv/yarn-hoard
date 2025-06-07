use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use crate::ApiClient;
use crate::atoms::Toaster;
use crate::pages::{ConfirmPage, LandingPage, LoginPage, SignupPage};

#[derive(Route, Clone)]
enum AppRoutes {
    #[to("/")]
    Landing,

    #[to("/signup")]
    Signup,

    #[to("/confirm")]
    ConfirmNoToken,

    #[to("/confirm/<token>")]
    Confirm(String),

    #[to("/login")]
    Login,

    #[not_found]
    NotFound,
}

#[component]
pub fn App() -> View {
    provide_context({
        let api_url = std::env!("API_URL");
        if api_url.contains("localhost") {
            ApiClient::insecure(api_url)
        } else {
            ApiClient::secure(api_url)
        }
    });

    provide_context(Toaster::new());
    let toaster = use_context::<Toaster>();

    view! {
        (toaster.view())
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| {
                view! {
                    div {
                        (match route.get_clone() {
                            AppRoutes::Landing => LandingPage(),
                            AppRoutes::Signup => SignupPage(),
                            AppRoutes::ConfirmNoToken => view! { ConfirmPage() },
                            AppRoutes::Confirm(token)=> view! { ConfirmPage(token=token) },
                            AppRoutes::Login => LoginPage(),
                            AppRoutes::NotFound => "404 Not Found".into(),
                        })
                    }
                }
            }
        )
    }
}
