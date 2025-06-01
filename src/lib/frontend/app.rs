use sycamore::prelude::*;
use sycamore_router::{HistoryIntegration, Route, Router};

use crate::frontend::ApiClient;
use crate::frontend::pages::{LandingPage, LoginPage, SignupPage};

#[derive(Route, Clone)]
enum AppRoutes {
    #[to("/")]
    Landing,
    #[to("/signup")]
    Signup,
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
            ApiClient::insecure(&api_url)
        } else {
            ApiClient::secure(&api_url)
        }
    });

    view! {
        Router(
            integration=HistoryIntegration::new(),
            view=|route: ReadSignal<AppRoutes>| {
                view! {
                    div {
                        (match route.get_clone() {
                            AppRoutes::Landing => LandingPage(),
                            AppRoutes::Signup => SignupPage(),
                            AppRoutes::Login => LoginPage(),
                            AppRoutes::NotFound => "404 Not Found".into(),
                        })
                    }
                }
            }
        )
    }
}
