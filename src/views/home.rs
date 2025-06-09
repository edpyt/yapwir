use leptos::prelude::*;

use crate::{
    components::{CountdownTimer, StartStopButton, TaskTitle},
    views::settings::{GearSvg, SettingsView},
    PomoConfig,
};

#[component]
pub fn HomeView(config: RwSignal<PomoConfig>) -> impl IntoView {
    view! {
        <button
            class="absolute top-10 right-10 btn btn-square btn-soft"
            onclick="my_modal_1.showModal()"
        >
            <GearSvg />
        </button>

        <div class="grid h-screen w-screen content-center justify-items-center gap-5">
            <TaskTitle title=config.read_untracked().task_title />
            <StartStopButton is_clicked_signal=config.read_untracked().pomo_state />
            <CountdownTimer config />
        </div>

        <dialog id="my_modal_1" class="modal">
            <div class="modal-box">
                <SettingsView config />
            </div>
        </dialog>
    }
}
