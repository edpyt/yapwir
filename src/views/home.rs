use leptos::prelude::*;

use crate::components::{CountdownTimer, StartStopButton};

#[component]
pub fn HomeView(pomo_state: RwSignal<bool>) -> impl IntoView {
    view! {
        <div class="grid h-screen w-screen content-center justify-items-center gap-5">
            <StartStopButton is_clicked_signal=pomo_state />
            <CountdownTimer timer_state=pomo_state />
        </div>
    }
}
