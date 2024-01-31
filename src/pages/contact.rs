use crate::components::modal_form::FormModal;
use leptos::*;


#[derive(Debug, Clone, PartialEq)]
pub struct ContactData {
    pub set_contact_first_name: WriteSignal<String>,
    pub set_contact_last_name: WriteSignal<String>,
    pub set_contact_email_addr: WriteSignal<String>,
    pub set_contact_phone: WriteSignal<String>,
}

#[component]
pub fn Contact() -> impl IntoView {
    let (contact_first_name, set_contact_first_name) = create_signal("".to_string());

    let (contact_last_name, set_contact_last_name) = create_signal("".to_string());

    let (contact_email_addr, set_contact_email_addr) = create_signal("".to_string());

    let (contact_phone, set_contact_phone) = create_signal("".to_string());

    let contact_data_setters = ContactData {
        set_contact_first_name,
        set_contact_last_name,
        set_contact_email_addr,
        set_contact_phone,
    };

    provide_context(contact_data_setters);


    view! {
        <>
            <h1>"Contact Form Modal"</h1>
            <FormModal/>

            <h2>"Contact Information"</h2>
            <h3>"Contact Card"</h3>

            <Contact_Card contact_first_name contact_last_name contact_email_addr contact_phone/>
        </>
    }
}


#[component]
fn Contact_Card(
    contact_first_name: ReadSignal<String>,
    contact_last_name: ReadSignal<String>,
    contact_email_addr: ReadSignal<String>,
    contact_phone: ReadSignal<String>,
) -> impl IntoView {
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

                <p>"First name: " <strong>{move || contact_first_name()}</strong></p>
                <p>"Last name: " <strong>{move || contact_last_name()}</strong></p>
                <p>"Email: " <strong>{move || contact_email_addr()}</strong></p>
                <p>"Phone: " <strong>{move || contact_phone()}</strong></p>

            </ErrorBoundary>
        </Transition>
    }
}
