use leptos::error::Result;
use leptos::*;

use gloo_net;
use serde::{Deserialize, Serialize};

#[component]
pub fn GetMessage() -> impl IntoView {
    let msg = create_local_resource(|| (), |_| async move { get_message().await });

    let data = move || match msg.get() {
        None => view! { <p>"Loading..."</p> }.into_view(),
        Some(data) => view! {
            <p>

                "The status code is: " <br/> <strong>{data.clone().map(|data| data.0)}</strong>
            </p>
            <p>"The message is: " <br/> <strong>{data.clone().map(|data| data.1)}</strong></p>
        }
        .into_view(),
    };


    let fallback = move |errors: RwSignal<Errors>| {
        let error_list = move || {
            errors.with(|errors| {
                errors
                    .iter()
                    .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                    .collect_view()
            })
        };
        view! {
            <div class="error">
                <h2>"Error"</h2>
                <ul>{error_list}</ul>
            </div>
        }
    };

    view! {
        <Transition fallback=move || {
            view! { <div>"Loading (Transition) Fallback..."</div> }
        }>
            <ErrorBoundary fallback=fallback>

                <h2>"Getting JSON data from a server"</h2>
                <p>{data()}</p>

            </ErrorBoundary>
        </Transition>
    }
}

async fn get_message() -> Result<(u16, String)> {
    let url = &format!("http://localhost:3000/api");

    let response = gloo_net::http::Request::get(url)
        .send()
        .await
        .expect("GET request to server failed");

    let status = response.status();

    let body = response.json::<Message>().await?;

    let msg = body.message;

    let _ = logging::log!("{}", status);
    let _ = logging::log!("{}", format!("{:?}", &msg));

    Ok((status, msg))
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    message: String,
}
