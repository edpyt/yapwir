use std::time::Duration;

use leptos::prelude::*;

use crate::{components::timer::TimerDurations, utils::convert_duration_to_hms_fn};

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

#[component]
pub fn SettingsView(
    pomo_state: RwSignal<bool>,
    timer_durations: RwSignal<TimerDurations>,
    task_title: RwSignal<String>,
) -> impl IntoView {
    let focus_duration = RwSignal::new(timer_durations.read_untracked().focus);
    let break_duration = RwSignal::new(timer_durations.read_untracked().r#break);

    view! {
        {move || {
            if *pomo_state.read() {
                TimerAlreadyStartedAlert().into_any()
            } else {
                view! {
                    <div>
                        <div class="pb-5">
                            <p>"Task Title:"</p>
                            <input
                                type="text"
                                placeholder="Type task title here"
                                class="input w-full"
                                prop:value=task_title
                                on:input:target=move |ev| task_title.set(ev.target().value())
                            />
                        </div>
                        <div>"Focus Duration:" <DurationSetupInput duration=focus_duration /></div>
                        <div>"Break Duration:" <DurationSetupInput duration=break_duration /></div>
                    </div>
                }
                    .into_any()
            }
        }}
        // FIXME: remove `modal` here
        <div class="modal-action">
            <Show when=move || !*pomo_state.read()>
                <button
                    class="btn btn-outline btn-success"
                    on:click=move |_| {
                        let mut timer_durations = timer_durations.write();
                        timer_durations.focus = *focus_duration.read();
                        timer_durations.r#break = *break_duration.read();
                    }
                >

                    Save
                </button>
            </Show>
            <form method="dialog">
                <button class="btn btn-outline btn-error">Close</button>
            </form>
        </div>
    }
}

#[component]
fn TimerAlreadyStartedAlert() -> impl IntoView {
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
}

#[component]
fn DurationSetupInput(duration: RwSignal<Duration>) -> impl IntoView {
    let (hours, minutes, seconds) = convert_duration_to_hms_fn(duration);

    let on_input_hms =
        move |ev: leptos::ev::Targeted<leptos::ev::Event, leptos::web_sys::HtmlInputElement>| {
            let new_value: u64 = ev.target().value().parse().expect("Can't parse as number");

            let new_duration = match ev.target().placeholder().as_str() {
                "H" => Duration::from_secs((new_value * 60 * 60) + (minutes() * 60) + seconds()),
                "M" => Duration::from_secs((hours() * 60 * 60) + (new_value * 60) + seconds()),
                "S" => Duration::from_secs((hours() * 60 * 60) + (minutes() * 60) + new_value),
                _ => todo!(),
            };

            duration.set(new_duration);
        };

    view! {
        <div class="flex gap-3">
            <div class="flex-1">
                <input
                    type="number"
                    class="input"
                    required
                    min="0"
                    placeholder="H"
                    prop:value=hours
                    on:input:target=on_input_hms
                />

            </div>
            <div class="flex-1">
                <input
                    type="number"
                    class="input"
                    required
                    min="0"
                    placeholder="M"
                    prop:value=minutes
                    on:input:target=on_input_hms
                />
            </div>
            <div class="flex-1">
                <input
                    type="number"
                    class="input"
                    required
                    min="0"
                    placeholder="S"
                    prop:value=seconds
                    on:input:target=on_input_hms
                />
            </div>
        </div>
    }
}
