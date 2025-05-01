use leptos::prelude::*;
use yapwir_ui::components::StartStopButton;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="grid h-screen w-screen content-center justify-items-center">
            <StartStopButton />
        </div>
    }
}
