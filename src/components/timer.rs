use std::thread::sleep;
use std::time::Duration;

use leptos::prelude::*;

const DEFAULT_TIMER_SECONDS: u64 = 50 * 60;

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
    Effect::watch(
        move || timer_state.get(),
        move |timer_state, _, _| {
            if *timer_state {
                // FIXME: https://docs.rs/tokio/latest/tokio/time/fn.interval.html
                loop {
                    duration.set(Duration::new(duration.read().as_secs() - 1, 0));
                    sleep(Duration::from_secs(1));
                }
            }
        },
        true,
    );
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {

    use std::time::Duration;

    use leptos::{prelude::*, task::tick, web_sys};
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    use super::CountdownTimer;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn timer_state() {
        // Arrange
        let pomo_state = RwSignal::new(false);
        let duration = RwSignal::new(Duration::new(2, 0));
        let document = document();
        let test_wrapper = document.create_element("section").unwrap();
        let _dispose = mount_to(
            test_wrapper.clone().unchecked_into(),
            move || view! { <CountdownTimer timer_state=pomo_state duration /> },
        );
        let div = test_wrapper
            .query_selector("div")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlButtonElement>();
        let hours = div.first_child().unwrap();
        let minutes = hours.next_sibling().unwrap();
        let seconds = minutes.next_sibling().unwrap();
        let seconds_before = seconds.text_content().unwrap();

        // Act
        *pomo_state.write() = true;
        tick().await;
        *pomo_state.write() = false;

        // Assert
        // assert_ne!(seconds_before, hms[2].get().to_string());
    }
}
