pub mod components;
pub mod utils;
pub mod views;

use std::time::Duration;

use components::timer::{TimerDurations, TimerMode};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use views::HomeView;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
pub struct SendNotificationArgs<'a> {
    title: &'a str,
    body: &'a str,
}

#[component]
pub fn App() -> impl IntoView {
    let config = PomoConfig::default();

    view! { <HomeView config /> }
}

pub struct PomoConfig {
    pomo_state: RwSignal<bool>,
    timer_durations: RwSignal<TimerDurations>,
    task_title: RwSignal<String>,
}

impl Default for PomoConfig {
    fn default() -> Self {
        PomoConfig {
            pomo_state: RwSignal::new(false),
            timer_durations: RwSignal::new(TimerDurations {
                mode: RwSignal::new(TimerMode::Focus),
                focus: Duration::new(3, 0),
                r#break: Duration::new(5, 0),
            }),
            task_title: RwSignal::new(String::new()),
        }
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::App;
    use leptos::prelude::*;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn app() {
        let document = document();
        let test_wrapper = document.create_element("section").unwrap();
        let _ = document.body().unwrap().append_child(&test_wrapper);
        let _dispose = mount_to(test_wrapper.clone().unchecked_into(), || view! { <App /> });
        let div = document.query_selector("div").unwrap().unwrap();
        let inner = div.inner_html();

        assert!(inner.contains(
            "<button class=\"btn btn-circle btn-soft p-[142px] text-2xl\">START</button>"
        ));
        assert!(inner.contains("H"));
        assert!(inner.contains("M"));
        assert!(inner.contains("S"));
    }
}
