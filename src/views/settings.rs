use std::time::Duration;

use leptos::prelude::*;

use crate::{components::timer::TimerDurations, utils::convert_duration_to_hms_fn};

#[component]
pub fn SettingsView(
    pomo_state: RwSignal<bool>,
    timer_durations: RwSignal<TimerDurations>,
) -> impl IntoView {
    let focus_duration = RwSignal::new(timer_durations.read().focus);
    let break_duration = RwSignal::new(timer_durations.read().r#break);

    let (hours_fc, minutes_fc, seconds_fc) = convert_duration_to_hms_fn(focus_duration);
    let (hours_bk, minutes_bk, seconds_bk) = convert_duration_to_hms_fn(break_duration);

    view! {
        {move || {
            if *pomo_state.read() {
                view! {
                    <div role="alert" class="alert alert-error">
                        <svg
                            xmlns="http://www.w3.org/2000/svg"
                            class="h-6 w-6 shrink-0 stroke-current"
                            fill="none"
                            viewBox="0 0 24 24"
                        >
                            <path
                                stroke-linecap="round"
                                stroke-linejoin="round"
                                stroke-width="2"
                                d="M10 14l2-2m0 0l2-2m-2 2l-2-2m2 2l2 2m7-2a9 9 0 11-18 0 9 9 0 0118 0z"
                            />
                        </svg>
                        <span>Error! Timer already started.</span>
                    </div>
                }
                    .into_any()
            } else {
                view! {
                    <div>
                        "Focus Duration:" <div class="flex gap-3">
                            <div class="flex-1">
                                <input
                                    type="number"
                                    class="input"
                                    required
                                    min="0"
                                    placeholder="H"
                                />
                            </div>
                            <div class="flex-1">
                                <input
                                    type="number"
                                    class="input"
                                    required
                                    min="0"
                                    placeholder="M"
                                />
                            </div>
                            <div class="flex-1">
                                <input
                                    type="number"
                                    class="input"
                                    required
                                    min="0"
                                    placeholder="S"
                                />
                            </div>
                        </div> "Break Duration:" <div class="flex gap-3">
                            <div class="flex-1">
                                <input
                                    type="number"
                                    class="input"
                                    required
                                    min="0"
                                    placeholder="H"
                                />
                            </div>
                            <div class="flex-1">
                                <input
                                    type="number"
                                    class="input"
                                    required
                                    min="0"
                                    placeholder="M"
                                />
                            </div>
                            <div class="flex-1">
                                <input
                                    type="number"
                                    class="input"
                                    required
                                    min="0"
                                    placeholder="S"
                                />
                            </div>
                        </div>
                    </div>
                }
                    .into_any()
            }
        }}
        // FIXME: remove `modal` here
        <div class="modal-action">
            <button class="btn btn-outline btn-success">Save</button>
            <form method="dialog">
                <button class="btn btn-outline btn-error">Close</button>
            </form>
        </div>
    }
}

#[component]
pub fn GearSvg() -> impl IntoView {
    // FIXME: change `fill` for svg
    view! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="white" class="size-6">
            <path
                fill-rule="evenodd"
                d="M11.078 2.25c-.917 0-1.699.663-1.85 1.567L9.05 4.889c-.02.12-.115.26-.297.348a7.493 7.493 0 0 0-.986.57c-.166.115-.334.126-.45.083L6.3 5.508a1.875 1.875 0 0 0-2.282.819l-.922 1.597a1.875 1.875 0 0 0 .432 2.385l.84.692c.095.078.17.229.154.43a7.598 7.598 0 0 0 0 1.139c.015.2-.059.352-.153.43l-.841.692a1.875 1.875 0 0 0-.432 2.385l.922 1.597a1.875 1.875 0 0 0 2.282.818l1.019-.382c.115-.043.283-.031.45.082.312.214.641.405.985.57.182.088.277.228.297.35l.178 1.071c.151.904.933 1.567 1.85 1.567h1.844c.916 0 1.699-.663 1.85-1.567l.178-1.072c.02-.12.114-.26.297-.349.344-.165.673-.356.985-.57.167-.114.335-.125.45-.082l1.02.382a1.875 1.875 0 0 0 2.28-.819l.923-1.597a1.875 1.875 0 0 0-.432-2.385l-.84-.692c-.095-.078-.17-.229-.154-.43a7.614 7.614 0 0 0 0-1.139c-.016-.2.059-.352.153-.43l.84-.692c.708-.582.891-1.59.433-2.385l-.922-1.597a1.875 1.875 0 0 0-2.282-.818l-1.02.382c-.114.043-.282.031-.449-.083a7.49 7.49 0 0 0-.985-.57c-.183-.087-.277-.227-.297-.348l-.179-1.072a1.875 1.875 0 0 0-1.85-1.567h-1.843ZM12 15.75a3.75 3.75 0 1 0 0-7.5 3.75 3.75 0 0 0 0 7.5Z"
                clip-rule="evenodd"
            />
        </svg>
    }
}
