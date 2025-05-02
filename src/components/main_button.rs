use leptos::prelude::*;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StartStopButtonText {
    Start,
    Stop,
}

impl StartStopButtonText {
    fn as_str(&self) -> &'static str {
        match self {
            StartStopButtonText::Start => "start",
            StartStopButtonText::Stop => "stop",
        }
    }
}

#[component]
pub fn StartStopButton(
    is_clicked_signal: RwSignal<bool>,
    #[prop(default = RwSignal::new(StartStopButtonText::Start))] btn_text_signal: RwSignal<
        StartStopButtonText,
    >,
) -> impl IntoView {
    let (is_clicked, set_is_clicked) = is_clicked_signal.split();
    let (btn_text, set_btn_text) = btn_text_signal.split();
    view! {
        <button
            class="btn btn-circle btn-soft p-[142px] text-2xl"
            on:click=move |_| {
                set_is_clicked.set(!is_clicked.get());
                *set_btn_text.write() = match btn_text.get() {
                    StartStopButtonText::Start => StartStopButtonText::Stop,
                    StartStopButtonText::Stop => StartStopButtonText::Start,
                };
            }
        >
            {move || btn_text.get().as_str().to_uppercase()}
        </button>
    }
}

#[cfg(test)]
mod tests {
    use leptos::prelude::*;
    use leptos::task::tick;
    use leptos::web_sys;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    use super::{StartStopButton, StartStopButtonText};

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    async fn test_click_button() {
        // Arrange
        let document = document();
        let is_clicked_signal = RwSignal::new(false);
        let btn_text_signal = RwSignal::new(StartStopButtonText::Start);
        let test_wrapper = document.create_element("section").unwrap();
        let _dispose = mount_to(
            test_wrapper.clone().unchecked_into(),
            move || view! { <StartStopButton is_clicked_signal btn_text_signal /> },
        );
        let btn = test_wrapper
            .query_selector("button")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlButtonElement>();

        // Assert
        assert_eq!(btn.text_content().unwrap(), "START");

        // Assert
        assert_eq!(btn.text_content().unwrap(), "START");

        // Act
        btn.click();
        tick().await;

        // Assert
        assert!(is_clicked_signal.get_untracked());
        assert_eq!(btn_text_signal.get_untracked(), StartStopButtonText::Stop);
        assert_eq!(btn.text_content().unwrap(), "STOP");
    }
}
