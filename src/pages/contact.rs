use crate::components::modal_form::FormModal;
use leptos::*;


#[component]
pub fn Contact() -> impl IntoView {
    view! {
        <>
            <h1>"Contact Form Modal"</h1>
            <FormModal/>

            <h2>"Contact Information"</h2>
            <h3>"Contact Card"</h3>

            <Contact_Card/>
        </>
    }
}


#[component]
fn Contact_Card() -> impl IntoView {
    let (first_name, set_first_name) = create_signal("".to_string());

    let (last_name, set_last_name) = create_signal("".to_string());

    let (email_addr, set_email_addr) = create_signal("".to_string());

    let (phone, set_phone) = create_signal("".to_string());

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

                <p>"First name: " {first_name()}</p>
                <p>"Last name: " {last_name()}</p>
                <p>"Email: " {email_addr()}</p>
                <p>"Phone: " {phone()}</p>

            </ErrorBoundary>
        </Transition>
    }
}
