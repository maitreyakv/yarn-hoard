use sycamore::prelude::*;

#[component(inline_props)]
pub fn Button(
    #[prop(setter(into))] children: Children,
    #[prop(setter(into))] disabled: Option<MaybeDyn<bool>>,
) -> View {
    let disabled = disabled.unwrap_or(MaybeDyn::Static(false));
    view! {
        button(class="button", disabled=disabled) {
            (children)
        }
    }
}
