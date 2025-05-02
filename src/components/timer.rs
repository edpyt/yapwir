use std::iter::zip;

use leptos::prelude::*;

#[component]
pub fn CountdownTimer(
    timer_state: RwSignal<bool>,
    #[prop(default = [
        RwSignal::new(0),
        RwSignal::new(50),
        RwSignal::new(0)
    ])]
    hms: [RwSignal<u32>; 3],
) -> impl IntoView {
    Effect::watch(
        move || timer_state.get(),
        move |timer_state, _, _| {
            match *timer_state {
                true => *hms[2].write() -= 1,
                false => *hms[1].write() -= 1,
            };
        },
        true,
    );
    let hms_zip = zip(["hours", "minutes", "seconds"], hms);

    view! {
        <div class="grid auto-cols-max grid-flow-col gap-5 text-center">
            {hms_zip
                .into_iter()
                .map(|(name, var)| {
                    view! {
                        <div class="flex flex-col">
                            <span class="countdown font-mono text-5xl">
                                <span
                                    style=move || format!("--value:{};", var.get())
                                    aria-live="polite"
                                    aria-label=move || var
                                >
                                    {var}
                                </span>
                            </span>
                            {name}

                        </div>
                    }
                })
                .collect_view()}
        </div>
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use leptos::{prelude::*, task::tick, web_sys};
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    use super::CountdownTimer;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn timer_state() {
        // Arrange
        let pomo_state = RwSignal::new(false);
        let hms = [RwSignal::new(0), RwSignal::new(50), RwSignal::new(0)];
        let document = document();
        let test_wrapper = document.create_element("section").unwrap();
        let _dispose = mount_to(
            test_wrapper.clone().unchecked_into(),
            move || view! { <CountdownTimer timer_state=pomo_state hms /> },
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
        assert_ne!(seconds_before, hms[2].get().to_string());
    }
}
