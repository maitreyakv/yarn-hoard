use sycamore::prelude::*;
use sycamore::web::tags::*;
use sycamore_router::{HistoryIntegration, Route, Router, RouterProps};

use crate::pages::{LandingPage, LoginPage, SignupPage};

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
            api_client::ApiClient::insecure(api_url)
        } else {
            api_client::ApiClient::secure(api_url)
        }
    });

    Router(RouterProps::new(
        HistoryIntegration::new(),
        |route: ReadSignal<AppRoutes>| {
            div()
                .children(move || match route.get_clone() {
                    AppRoutes::Landing => LandingPage(),
                    AppRoutes::Signup => SignupPage(),
                    AppRoutes::Login => LoginPage(),
                    AppRoutes::NotFound => "404 Not Found".into(),
                })
                .into()
        },
    ))
}
