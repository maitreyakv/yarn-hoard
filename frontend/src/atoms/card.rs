use std::sync::Once;

use sycamore::prelude::*;

use crate::util::inject_style_tag_into_document_head;

static INJECT_STYLE: Once = Once::new();

#[component(inline_props)]
pub fn Card(#[prop(setter(into))] children: Children) -> View {
    INJECT_STYLE.call_once(|| {
        inject_style_tag_into_document_head(include_str!("card.css"));
    });
    view! {
        div(class="card") {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn CardHeader(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="card-header") {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn CardTitle(#[prop(setter(into))] text: String) -> View {
    view! {
        div(class="card-title") {
            (text)
        }
    }
}

#[component(inline_props)]
pub fn CardContent(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="card-content") {
            (children)
        }
    }
}

#[component(inline_props)]
pub fn CardFooter(#[prop(setter(into))] children: Children) -> View {
    view! {
        div(class="card-footer") {
            (children)
        }
    }
}
