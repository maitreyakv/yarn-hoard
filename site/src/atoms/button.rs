use sycamore::prelude::*;
use sycamore::web::tags::button;

#[component(inline_props)]
pub fn Button(
    #[prop(setter(into))] children: Children,
    #[prop(setter(into))] disabled: Option<MaybeDyn<bool>>,
) -> View {
    button()
        .class("button")
        .disabled(disabled.unwrap_or(MaybeDyn::Static(false)))
        .children(children)
        .into()
}
