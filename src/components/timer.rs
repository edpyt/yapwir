use std::time::Duration;

use leptos::prelude::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimerMode {
    Focus,
    Break,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimerDurations {
    mode: RwSignal<TimerMode>,
    focus: Duration,
    r#break: Duration,
}

impl TimerDurations {
    fn get_duration(&self) -> Duration {
        match self.mode.get() {
            TimerMode::Focus => self.focus,
            TimerMode::Break => self.r#break,
        }
    }

    fn change_mode(&mut self) {
        *self.mode.write() = match self.mode.get() {
            TimerMode::Focus => TimerMode::Break,
            TimerMode::Break => TimerMode::Focus,
        }
    }
}

#[component]
pub fn CountdownTimer(
    timer_state: RwSignal<bool>,
    #[prop(default=RwSignal::new(TimerDurations {
        mode: RwSignal::new(TimerMode::Focus),
        focus: Duration::new(5,0),
        r#break: Duration::new(5,0),
    }))]
    timer_durations: RwSignal<TimerDurations>,
) -> impl IntoView {
    let duration = RwSignal::new(timer_durations.get_untracked().get_duration());
    create_timer_state_event(timer_state, timer_durations, duration);

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
                "H"
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
                "M"
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
                "S"
            </div>
        </div>

        {move || match timer_durations.get().mode.get() {
            TimerMode::Focus => "FOCUS!",
            TimerMode::Break => "break",
        }}
    }
}

fn create_timer_state_event(
    timer_state: RwSignal<bool>,
    timer_durations: RwSignal<TimerDurations>,
    duration: RwSignal<Duration>,
) {
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
                    timer_state.set(false);
                    timer_durations.write().change_mode();
                    duration.set(timer_durations.get_untracked().get_duration());
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

    use crate::components::timer::TimerMode;

    use super::{CountdownTimer, TimerDurations};

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn timer_state() {
        // Arrange
        let pomo_state = RwSignal::new(false);
        let duration = Duration::new(2, 0);
        let timer_durations = RwSignal::new(TimerDurations {
            mode: RwSignal::new(TimerMode::Focus),
            focus: duration,
            r#break: duration,
        });
        let _duration_before = duration;
        let document = document();
        let test_wrapper = document.create_element("section").unwrap();
        let _dispose = mount_to(
            test_wrapper.clone().unchecked_into(),
            move || view! { <CountdownTimer timer_state=pomo_state timer_durations /> },
        );

        // Act
        *pomo_state.write() = true;
        tick().await;

        // Assert
        // TODO: need to sleep in test?
        // assert_ne!(
        //     duration.get_untracked().as_micros(),
        //     duration_before.as_micros()
        // );

        // Assert
        // TODO: if the timer has expired - change mode
        // let TimerMode::Break = timer_durations.get_untracked().mode.get_untracked() else {
        //     panic!("Expected Break enum variant.")
        // };
    }
}
