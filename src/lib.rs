pub mod components;

use components::{CountdownTimer, StartStopButton};
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let pomo_state = RwSignal::new(false);

    view! {
        <div class="grid h-screen w-screen content-center justify-items-center gap-5">
            <StartStopButton is_clicked_signal=pomo_state />
            <CountdownTimer timer_state=pomo_state />
        </div>
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
        assert!(inner.contains("hours"));
        assert!(inner.contains("minutes"));
        assert!(inner.contains("seconds"));
    }
}
