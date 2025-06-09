use std::time::Duration;

use leptos::{prelude::*, task::spawn_local};
use serde_wasm_bindgen::to_value;

use crate::{invoke, utils::convert_duration_to_hms_fn, PomoConfig, SendNotificationArgs};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TimerMode {
    Focus,
    Break,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerDurations {
    pub mode: RwSignal<TimerMode>,
    pub focus: Duration,
    pub r#break: Duration,
}

impl TimerDurations {
    fn get_duration(&self) -> Duration {
        match self.mode.get_untracked() {
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
pub fn CountdownTimer(config: RwSignal<PomoConfig>) -> impl IntoView {
    let duration = RwSignal::new(
        config
            .read_untracked()
            .timer_durations
            .get_untracked()
            .get_duration(),
    );
    let (hours, minutes, seconds) = convert_duration_to_hms_fn(duration);
    create_timer_state_event(config, duration);

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
                "H"
            </div>
            // TODO: use forloop
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

        {move || match config.read().timer_durations.get().mode.get() {
            TimerMode::Focus => "FOCUS!",
            TimerMode::Break => "break",
        }}
    }
}

fn create_timer_state_event(config: RwSignal<PomoConfig>, duration: RwSignal<Duration>) {
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
                    let config = config.read();
                    config.pomo_state.set(false);
                    config.timer_durations.write().change_mode();
                    duration.set(config.timer_durations.get_untracked().get_duration());

                    spawn_local(async move {
                        let args = to_value(&SendNotificationArgs {
                            title: "Session end.",
                            body: match config.timer_durations.get_untracked().mode.get_untracked()
                            {
                                TimerMode::Focus => "U need to focus",
                                TimerMode::Break => "U can rest now",
                            },
                        })
                        .unwrap();
                        invoke("send_notification", args).await;
                    });
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
        move || config.read().pomo_state.get(),
        move |timer_state, _, _| match *timer_state {
            true => start(),
            false => stop(),
        },
        true,
    );

    Effect::watch(
        move || config.read().timer_durations.get(),
        move |timer_durations, _, _| duration.set(timer_durations.get_duration()),
        false,
    );

    on_cleanup(stop);
}
