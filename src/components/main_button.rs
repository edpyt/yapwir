use leptos::prelude::*;

#[component]
pub fn StartStopButton() -> impl IntoView {
    view! { <button class="btn btn-circle btn-soft p-[142px] text-2xl">START</button> }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;

    use super::StartStopButton;

    wasm_bindgen_test_configure!(run_in_browser);

    #[wasm_bindgen_test]
    fn test_click_start_button() {
        todo!()
    }
}
