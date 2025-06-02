use std::time::Duration;

use leptos::prelude::*;

// TODO: need to return array here
pub fn convert_duration_to_hms_fn(
    duration: RwSignal<Duration>,
) -> (
    impl Fn() -> u64 + Copy,
    impl Fn() -> u64 + Copy,
    impl Fn() -> u64 + Copy,
) {
    let seconds = move || duration.read().as_secs() % 60;
    let minutes = move || (duration.read().as_secs() / 60) % 60;
    let hours = move || (duration.read().as_secs() / 60) / 60;

    (hours, minutes, seconds)
}
