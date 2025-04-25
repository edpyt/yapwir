use leptos::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_test::*;

#[wasm_bindgen_test]
fn test_pomo_button_start() {
    let document = document();
    let test_wrapper = document.create_element("section").unwrap();
    let _dispose = mount_to(
        test_wrapper.clone().unchecked_into(),
        || view! { <StartStopButton /> },
    );
}
