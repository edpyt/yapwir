use leptos::prelude::*;

#[component]
pub fn StartStopButton(is_clicked_signal: RwSignal<bool>) -> impl IntoView {
    view! {
        <button
            class="btn btn-circle btn-soft p-[142px] text-2xl"
            on:click=move |_| is_clicked_signal.set(!is_clicked_signal.get())
        >
            {move || if !is_clicked_signal.get() { "START" } else { "STOP" }}
        </button>
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use leptos::prelude::*;
    use leptos::task::tick;
    use leptos::web_sys;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    use super::StartStopButton;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    async fn test_click_button() {
        // Arrange
        let document = document();
        let is_clicked_signal = RwSignal::new(false);
        let test_wrapper = document.create_element("section").unwrap();
        let _dispose = mount_to(
            test_wrapper.clone().unchecked_into(),
            move || view! { <StartStopButton is_clicked_signal /> },
        );
        let btn = test_wrapper
            .query_selector("button")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlButtonElement>();

        // Assert
        assert_eq!(btn.text_content().unwrap(), "START");

        // Act
        btn.click();
        tick().await;

        // Assert
        assert!(is_clicked_signal.get_untracked());
        assert_eq!(btn.text_content().unwrap(), "STOP");
    }
}
