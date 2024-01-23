use crate::components::modal::Modal;
use crate::components::select_dropdown::SelectOption;

use leptos::*;


#[component]
pub fn Examples() -> impl IntoView {
    view! {
        <>
            <SelectOption/>

            <hr/>

            <h2>"The Leptos <Portal/> Component"</h2>
            <Modal/>
        </>
    }
}
