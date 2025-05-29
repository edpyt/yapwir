use leptos::prelude::*;

use crate::{
    components::{CountdownTimer, StartStopButton},
    views::settings::{GearSvg, SettingsView},
};

#[component]
pub fn HomeView(pomo_state: RwSignal<bool>) -> impl IntoView {
    view! {
        <button
            class="absolute top-10 right-10 btn btn-square btn-soft"
            onclick="my_modal_1.showModal()"
        >
            <GearSvg />
        </button>

        <div class="grid h-screen w-screen content-center justify-items-center gap-5">
            <StartStopButton is_clicked_signal=pomo_state />
            <CountdownTimer timer_state=pomo_state />
        </div>

        <dialog id="my_modal_1" class="modal">
            <div class="modal-box">
                <SettingsView />
                <div class="modal-action">
                    <form method="dialog">
                        <button class="btn btn-outline btn-error">X</button>
                    </form>
                </div>
            </div>
        </dialog>
    }
}
