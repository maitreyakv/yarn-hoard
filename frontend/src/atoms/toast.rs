use std::rc::Rc;
use std::sync::Once;

use sycamore::prelude::*;

use crate::atoms::Icon;
use crate::class;
use crate::util::inject_style_tag_into_document_head;

static INJECT_STYLE: Once = Once::new();

#[derive(Clone, Copy)]
pub struct Toaster {
    toast: Signal<Option<Toast>>,
}

impl Toaster {
    pub fn new() -> Self {
        Self {
            toast: create_signal(None),
        }
    }

    pub fn warn(&self, msg: String) {
        self.toast.set(Some(Toast::Warning(Rc::from(msg))))
    }

    pub fn view(&self) -> View {
        view! {
            ToasterView(toast=*self.toast)
        }
    }
}

#[component(inline_props)]
fn ToasterView(toast: ReadSignal<Option<Toast>>) -> View {
    INJECT_STYLE.call_once(|| {
        inject_style_tag_into_document_head(include_str!("toast.css"));
    });

    view! {
        div(class="toaster") {
            (if let Some(toast) = toast.get_clone() {toast.view()} else {view!{}})
        }
    }
}

#[derive(Clone)]
enum Toast {
    Warning(Rc<str>),
}

impl Toast {
    fn view(&self) -> View {
        match self {
            Self::Warning(text) => {
                let text = text.to_string();
                view! {
                     ToastView(class="toast-warning") {
                        (Icon::Warning.view())
                        (text)
                    }
                }
            }
        }
    }
}

#[component(inline_props)]
fn ToastView(class: &'static str, #[prop(setter(into))] children: Children) -> View {
    let visible = create_signal(true);

    sycamore::futures::spawn_local(async move {
        gloo::timers::future::sleep(std::time::Duration::from_secs(3)).await;
        if visible.is_alive() {
            visible.set(false);
        }
    });

    let class = move || {
        if visible.get() {
            class!("toast", "toast-show", class)
        } else {
            class!("toast", "toast-hide", class)
        }
    };

    view! {
        div(class=class) {
            (children)
        }
    }
}
