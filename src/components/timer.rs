use std::time::Duration;

use leptos::prelude::*;

const DEFAULT_TIMER_SECONDS: u64 = 5;

#[component]
pub fn CountdownTimer(
    timer_state: RwSignal<bool>,
    #[prop(default = RwSignal::new(Duration::new(DEFAULT_TIMER_SECONDS, 0)))] duration: RwSignal<
        Duration,
    >,
) -> impl IntoView {
    create_timer_state_event(timer_state, duration);

    let seconds = move || duration.read().as_secs() % 60;
    let minutes = move || (duration.read().as_secs() / 60) % 60;
    let hours = move || (duration.read().as_secs() / 60) / 60;

    view! {
        <div class="grid auto-cols-max grid-flow-col gap-5 text-center">
            // TODO: use forloop
            <div class="flex flex-col">
                <span class="countdown font-mono text-5xl">
                    <span
                        style=move || format!("--value:{};", hours())
                        aria-live="polite"
                        aria-label=hours
                    >
                        {hours}
                    </span>
                </span>
                "hours"
            </div>
            <div class="flex flex-col">
                <span class="countdown font-mono text-5xl">
                    <span
                        style=move || format!("--value:{};", minutes())
                        aria-live="polite"
                        aria-label=minutes
                    >
                        {minutes}
                    </span>
                </span>
                "minutes"
            </div>
            <div class="flex flex-col">
                <span class="countdown font-mono text-5xl">
                    <span
                        style=move || format!("--value:{};", seconds())
                        aria-live="polite"
                        aria-label=seconds
                    >
                        {seconds}
                    </span>
                </span>
                "seconds"
            </div>

        </div>
    }
}

fn create_timer_state_event(timer_state: RwSignal<bool>, duration: RwSignal<Duration>) {
    let interval_handle: RwSignal<Option<IntervalHandle>> = RwSignal::new(None);
    let stop = move || {
        if let Some(handle) = interval_handle.get_untracked() {
            handle.clear();
            interval_handle.set(None);
        }
    };

    let create_handle = move || {
        set_interval_with_handle(
            move || {
                if duration.read().as_secs() > 0 {
                    duration.update(|duration| {
                        *duration = duration.saturating_sub(Duration::from_secs(1))
                    })
                } else {
                    stop();
                    timer_state.set(false);
                }
            },
            Duration::from_secs(1),
        )
        .expect("could not create interval")
    };
    let start = move || {
        if let Some(handle) = interval_handle.get_untracked() {
            handle.clear();
        };
        interval_handle.set(Some(create_handle()))
    };

    Effect::watch(
        move || timer_state.get(),
        move |timer_state, _, _| match *timer_state {
            true => start(),
            false => stop(),
        },
        true,
    );

    on_cleanup(stop);
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {

    use std::time::Duration;

    use leptos::{prelude::*, task::tick};
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    use super::CountdownTimer;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn timer_state() {
        // Arrange
        let pomo_state = RwSignal::new(false);
        let duration = RwSignal::new(Duration::new(2, 0));
        let _duration_before = duration.get_untracked();
        let document = document();
        let test_wrapper = document.create_element("section").unwrap();
        let _dispose = mount_to(
            test_wrapper.clone().unchecked_into(),
            move || view! { <CountdownTimer timer_state=pomo_state duration /> },
        );

        // Act
        *pomo_state.write() = true;
        tick().await;

        // Assert
        // FIXME:
        // assert_ne!(
        //     duration.get_untracked().as_micros(),
        //     duration_before.as_micros()
        // );
    }
}
