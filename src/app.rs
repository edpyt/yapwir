use leptos::prelude::*;
use yapwir_ui::components::StartStopButton;

#[component]
pub fn App() -> impl IntoView {
    let is_clicked_signal = RwSignal::new(false);

    view! {
        <div class="grid h-screen w-screen content-center justify-items-center">
            <StartStopButton is_clicked_signal />
        </div>
    }
}
