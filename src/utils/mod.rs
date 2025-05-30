use std::time::Duration;

use leptos::prelude::*;

// TODO: need to return array here
pub fn convert_duration_to_hms_fn(duration: RwSignal<Duration>) -> [dyn Fn() -> u64; 3] {
    let seconds = move || duration.read().as_secs() % 60;
    let minutes = move || (duration.read().as_secs() / 60) % 60;
    let hours = move || (duration.read().as_secs() / 60) / 60;

    [hours, minutes, seconds]
}

#[component]
pub fn TestView {
    let duration = Duration::new(42,0);
    let [hours,minutes,seconds] = convert_duration_to_hms_fn(duration);

    view!{ 
        {hours}
        {minutes}
        {seconds}
    }
}
