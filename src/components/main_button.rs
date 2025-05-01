use leptos::prelude::*;
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StartStopButtonText {
    Start,
    Stop,
    // TODO: add `resume` text
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
    is_clicked_signal: (ReadSignal<bool>, WriteSignal<bool>),
    #[prop(default = signal(StartStopButtonText::Start))] btn_text_signal: (
        ReadSignal<StartStopButtonText>,
        WriteSignal<StartStopButtonText>,
    ),
) -> impl IntoView {
    let (is_clicked, set_is_clicked) = is_clicked_signal;
    let (btn_text, set_btn_text) = btn_text_signal;
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
    use leptos::web_sys;
    use wasm_bindgen::JsCast;
    use wasm_bindgen_test::*;

    use super::{StartStopButton, StartStopButtonText};

    wasm_bindgen_test_configure!(run_in_browser);

    #[allow(dead_code)]
    #[wasm_bindgen_test]
    fn test_click_button() {
        // Arrange
        let document = document();
        let is_clicked_signal = signal(false);
        let btn_text_signal = signal(StartStopButtonText::Start);
        let test_wrapper = document.create_element("section").unwrap();
        let _ = document.body().unwrap().append_child(&test_wrapper);
        let _dispose = mount_to(
            test_wrapper.clone().unchecked_into(),
            move || view! { <StartStopButton is_clicked_signal btn_text_signal /> },
        );
        let btn = test_wrapper
            .query_selector("button")
            .unwrap()
            .unwrap()
            .unchecked_into::<web_sys::HtmlElement>();

        // Act
        btn.click();

        // Assert
        assert!(is_clicked_signal.0.get_untracked());
        assert_eq!(btn_text_signal.0.get_untracked(), StartStopButtonText::Stop);
        // FIXME:
        // assert_eq!(btn.text_content().unwrap(), "STOP");
    }
}
