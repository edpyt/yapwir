use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="grid h-screen w-screen content-center justify-items-center">
            <StartStopButton />
        </div>
    }
}

#[component]
pub fn StartStopButton() -> impl IntoView {
    view! { <button class="btn btn-circle btn-soft p-[142px] text-2xl">START</button> }
}
