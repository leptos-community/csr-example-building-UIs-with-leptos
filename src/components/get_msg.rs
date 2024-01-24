use leptos::error::Result;
use leptos::*;
use reqwasm;

#[component]
pub fn GetMessage() -> impl IntoView {
    let msg = create_local_resource(|| (), |_| async move { load_data().await });


    view! {
        <Transition fallback=move || {
            view! { <div>"Loading (Transition) Fallback..."</div> }
        }>
            <ErrorBoundary fallback=move |_| {
                view! { <div>"Loading (ErrorBoundary) Fallback..."</div> }
            }>

                <p>"Getting secret message from the server..."</p>
                <p>
                    "your message is: " // {msg}
                    "<Secret Message>"
                </p>
            </ErrorBoundary>
        </Transition>
    }
}

async fn load_data() -> Result<reqwasm::http::Response> {
    let response = reqwasm::http::Request::get(&format!("http://localhost:3000/api"))
        .send()
        .await?;

    // let _ = move || logging::log!(format!("{}", response.to_string()));
    // let res = response.

    // "Hello from load_data function".to_string()
    Ok(response)
}
