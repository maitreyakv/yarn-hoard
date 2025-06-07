use anyhow::anyhow;
use sycamore::prelude::*;
use tracing::{debug, error};

#[tracing::instrument]
pub fn inject_style_tag_into_document_head(style: &str) {
    let inject = || {
        let style_element = document()
            .create_element("style")
            .map_err(|_| anyhow!("Failed to create a style element!"))?;
        style_element.set_inner_html(style);
        let _ = document()
            .head()
            .ok_or_else(|| anyhow!("Document is missing head tag!"))?
            .append_child(&style_element)
            .map_err(|_| anyhow!("Failed to append the new style tag to the head tag!"));
        Ok::<(), anyhow::Error>(())
    };

    if let Err(error) = inject() {
        error!(?error);
    } else {
        debug!(?style, "Injected style tag into header");
    }
}

#[macro_export]
macro_rules! class (
    // Match a single string literal
    ($first:expr) => {
        $first.to_string()
    };
    // Match multiple string literals
    ($first:expr, $($rest:expr),+) => {
        format!("{} {}", $first, class!($($rest),+))
    };
);
