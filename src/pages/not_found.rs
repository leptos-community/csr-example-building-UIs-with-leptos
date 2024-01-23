use leptos::*;

/// 404 Not Found Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="page_container">
            <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>
        </div>
    }
}
