use leptos::prelude::*;

use crate::{
    components::{CountdownTimer, StartStopButton, TaskTitle},
    views::settings::{GearSvg, SettingsView},
    PomoConfig,
};

#[component]
pub fn HomeView(config: PomoConfig) -> impl IntoView {
    view! {
        <button
            class="absolute top-10 right-10 btn btn-square btn-soft"
            onclick="my_modal_1.showModal()"
        >
            <GearSvg />
        </button>

        <div class="grid h-screen w-screen content-center justify-items-center gap-5">
            <TaskTitle title=config.task_title />
            <StartStopButton is_clicked_signal=config.pomo_state />
            <CountdownTimer timer_state=config.pomo_state timer_durations=config.timer_durations />
        </div>

        <dialog id="my_modal_1" class="modal">
            <div class="modal-box">
                <SettingsView
                    pomo_state=config.pomo_state
                    timer_durations=config.timer_durations
                    task_title=config.task_title
                />
            </div>
        </dialog>
    }
}
