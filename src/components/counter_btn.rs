use leptos::prelude::*;

/// A parameterized incrementing button
#[component]
pub fn Button(#[prop(into)]title: String) -> impl IntoView {
    view! {
        <button>
            {title}
        </button>
    }
}
