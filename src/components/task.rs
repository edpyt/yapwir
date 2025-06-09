//! Session Task Title

use leptos::prelude::*;

#[component]
pub fn TaskTitle(title: RwSignal<String>) -> impl IntoView {
    view! { <p class="text-2xl text-red-400">{title}</p> }
}
