use crate::components::modal_form::FormModal;
use leptos::*;


#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <div class="page_container">
            <h1>"Contact Form Modal"</h1>
            <FormModal/>
        </div>
    }
}
