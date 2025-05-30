use leptos::prelude::*;
use std::time::Duration;

use crate::{
    components::{
        timer::{TimerDurations, TimerMode},
        CountdownTimer, StartStopButton,
    },
    views::settings::{GearSvg, SettingsView},
};

#[component]
pub fn HomeView(
    pomo_state: RwSignal<bool>,
    #[prop(default=RwSignal::new(TimerDurations {
        mode: RwSignal::new(TimerMode::Focus),
        focus: Duration::new(3,0),
        r#break: Duration::new(5,0),
    }))]
    timer_durations: RwSignal<TimerDurations>,
) -> impl IntoView {
    view! {
        <button
            class="absolute top-10 right-10 btn btn-square btn-soft"
            onclick="my_modal_1.showModal()"
        >
            <GearSvg />
        </button>

        <div class="grid h-screen w-screen content-center justify-items-center gap-5">
            <StartStopButton is_clicked_signal=pomo_state />
            <CountdownTimer timer_state=pomo_state timer_durations />
        </div>

        <dialog id="my_modal_1" class="modal">
            <div class="modal-box">
                <SettingsView pomo_state timer_durations />
            </div>
        </dialog>
    }
}
