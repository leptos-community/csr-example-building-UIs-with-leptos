use crate::{pages::contact::ContactData, BASE_API_URL};

use leptos::html::Div;
use leptos::*;
use leptos_router::Form;

use leptos_use::on_click_outside;

use gloo_net;
use js_sys::wasm_bindgen::UnwrapThrowExt;
use regex_lite::Regex;
use serde::{Deserialize, Serialize};
use web_sys::{KeyboardEvent, SubmitEvent};


#[component]
pub fn FormModal() -> impl IntoView {
    // --- Set Up Modal (with 'click outside modal to dismiss' behaviour) ---
    let (show_modal, set_show_modal) = create_signal(false);


    // Dismiss modal when "Escape" (or 'q') key is pressed
    let dismiss_modal_with_keyboard = window_event_listener(ev::keydown, move |ev| {
        if ev.key() == "Escape" || ev.key() == "q" || ev.key() == "Q" {
            set_show_modal(false);
        }
    });
    on_cleanup(move || dismiss_modal_with_keyboard.remove());


    // use `Leptos-Use` to setup modal 'click outside modal to dismiss' behaviour
    let modal_target: NodeRef<Div> = create_node_ref::<Div>();
    on_cleanup(on_click_outside(modal_target, move |_| {
        set_show_modal(false)
    }));

    // --- End Modal Setup ---


    view! {
        <button id="btn-show" type="button" on:click=move |_| set_show_modal(true)>
            "Contact Us"
        </button>

        <Show when=show_modal fallback=|| ()>
            <Portal mount=document().get_element_by_id("portal_root").unwrap()>
                <div class="modal_background">
                    <div _ref=modal_target>
                        <dialog class="modal_content" open=show_modal>

                            <div>
                                <button id="btn-hide" on:click=move |_| set_show_modal(false)>
                                    "Close ❌"
                                </button>

                                <ModalBody set_show_modal/>

                            </div>

                        </dialog>
                    </div>
                </div>
            </Portal>
        </Show>
    }
}


#[component]
fn ModalBody(set_show_modal: WriteSignal<bool>) -> impl IntoView {
    // --- Modal Body ---

    // --- Contact Form ---

    // Setup name fields
    let (first_name, set_first_name) = create_signal("".to_string());
    let (last_name, set_last_name) = create_signal("".to_string());

    let on_first_name_input = move |ev| {
        set_first_name.update(move |name| *name = event_target_value(&ev));
    };
    let on_last_name_input = move |ev| {
        set_last_name.update(move |name| *name = event_target_value(&ev));
    };

    let first_name_form_len = move || first_name().len();
    let last_name_form_len = move || last_name().len();


    // --- email address form, with email address checked by regex_lite ---
    let (email_addr, set_email_addr) = create_signal("".to_string());

    // Update email addr form field
    let on_email_input = move |ev| {
        set_email_addr.update(move |email| *email = event_target_value(&ev));
    };

    // test if email addr conforms
    let check_email_resource =
        create_local_resource(email_addr, |email| async move { check_email_regex(email) });

    let is_email_good = move || check_email_resource.get().unwrap_or_else(|| false);

    let email_form_len = move || email_addr().len();


    // --- Telephone input, checked locally by regex_lite ---
    let (phone, set_phone) = create_signal("".to_string());

    let on_phone_input = move |ev| {
        set_phone.update(move |phone| *phone = event_target_value(&ev));
    };

    // test if phone number conforms
    let check_phone_number_resource =
        create_local_resource(phone, |phone| async move { check_phone_regex(phone) });

    let is_phone_number_good = move || check_phone_number_resource.get().unwrap_or_else(|| false);

    let phone_form_len = move || phone().len();

    // Disable submit button until contact form meets requirements
    let check_form_meets_requirements_resource = create_local_resource(
        move || {
            (
                first_name_form_len(),
                last_name_form_len(),
                email_form_len(),
                phone_form_len(),
            )
        },
        move |val| async move { check_form_meets_minimum_requirements(val.0, val.1, val.2, val.3) },
    );

    let form_meets_reqs = move || {
        check_form_meets_requirements_resource
            .get()
            .unwrap_or_else(|| false)
    };


    // set up custom on:submit for contact form
    let contact_form_ref: NodeRef<html::Form> = create_node_ref();

    let setters =
        use_context::<ContactData>().expect("should have found the setter provided by context");


    let submit = move || {
        let contact_form = contact_form_ref
            .get()
            .expect("Couldn't get reference to form");


        let contact_form_data = web_sys::FormData::new_with_form(&contact_form).unwrap_throw();

        let action = contact_form
            .get_attribute("action")
            .unwrap_or_default()
            .to_lowercase();

        if form_meets_reqs() {
            spawn_local(async move {
                let res = gloo_net::http::Request::post(&action)
                    .header("Accept", "application/json")
                    .body(contact_form_data)
                    .unwrap()
                    .send()
                    .await;


                if let Ok(data) = res {
                    let results = data.json::<Contact>().await.expect("couldn't parse json");

                    setters
                        .set_contact_first_name
                        .update(|val| *val = results.first_name);
                    setters
                        .set_contact_last_name
                        .update(|val| *val = results.last_name);
                    setters
                        .set_contact_email_addr
                        .update(|val| *val = results.email);
                    setters.set_contact_phone.update(|val| *val = results.phone);
                } else {
                    logging::error!("<Form/> error while POSTing contact data");
                }
            });
            set_show_modal(false);
        }
    };

    let on_submit = move |ev: SubmitEvent| {
        // don't go to api page..
        ev.prevent_default();

        submit();
    };


    // submit the form when pressing "enter" key, even if not currently focused on an input field
    let submit_btn_ref = create_node_ref::<html::Button>();

    let submit_form_enter_key_listener =
        window_event_listener(ev::keydown, move |ev: KeyboardEvent| {
            if ev.key() == "Enter" {
                if form_meets_reqs() {
                    submit();
                }
            }
        });
    on_cleanup(move || submit_form_enter_key_listener.remove());

    // --- END contact form ---

    view! {
        <aside class="modal_body">

            <h2 class="modal_form_title">"Contact Us"</h2>

            <Form
                attr:id="contact_form"
                method="POST"
                action=format!("{}/api/contact", BASE_API_URL)
                on:submit=on_submit
                node_ref=contact_form_ref
            >

                <fieldset class="contact_form_fieldset" form="contact_form" name="contact_form">

                    <div class="contact_input">
                        <label class="input_label" for="first_name">
                            "First name:"
                        </label>

                        <input
                            class="contact_input"
                            name="first_name"
                            id="first_name"
                            type="text"
                            placeholder="Johnny"
                            prop:value=first_name
                            on:input=on_first_name_input
                        />

                        <Suspense fallback=|| view! { format!("{}", "🤔".to_string()) }>
                            <span>
                                {move || {
                                    if first_name_form_len() == 0 {
                                        " *"
                                    } else if first_name_form_len() > 2 {
                                        " ✅"
                                    } else {
                                        " ❌"
                                    }
                                }}

                            </span>
                        </Suspense>
                    </div>

                    <br/>

                    <div class="contact_input">
                        <label class="input_label" for="last_name">
                            "Last name:"
                        </label>

                        <input
                            name="last_name"
                            id="last_name"
                            type="text"
                            placeholder="Appleseed"
                            prop:value=last_name
                            on:input=on_last_name_input
                        />

                        <Suspense fallback=|| view! { format!("{}", "🤔".to_string()) }>
                            <span>
                                {move || {
                                    if last_name_form_len() == 0 {
                                        " *"
                                    } else if last_name_form_len() > 2 {
                                        " ✅"
                                    } else {
                                        " ❌"
                                    }
                                }}

                            </span>
                        </Suspense>
                    </div>

                    <br/>

                    <div class="contact_input">
                        <label class="input_label" for="email">
                            "Email address:"
                        </label>

                        <input
                            name="email"
                            id="email"
                            type="email"
                            placeholder="your.email@website.com"
                            prop:value=email_addr
                            on:input=on_email_input
                        />

                        <Suspense fallback=|| view! { format!("{}", "🤔".to_string()) }>
                            <span>
                                {move || {
                                    if email_form_len() == 0 {
                                        " *"
                                    } else if is_email_good() {
                                        " ✅"
                                    } else {
                                        " ❌"
                                    }
                                }}

                            </span>
                        </Suspense>
                    </div>

                    <br/>

                    <div class="contact_input">
                        <label class="input_label" for="phone">
                            "Phone number:"
                        </label>

                        <input
                            name="phone"
                            id="phone"
                            type="tel"
                            placeholder="1-250-555-5555"
                            prop:value=phone
                            on:input=on_phone_input
                        />

                        <Suspense fallback=|| view! { format!("{}", "🤔".to_string()) }>
                            <span>
                                {move || {
                                    if phone_form_len() == 0 {
                                        " *"
                                    } else if is_phone_number_good() {
                                        " ✅"
                                    } else {
                                        " ❌"
                                    }
                                }}

                            </span>
                        </Suspense>
                    </div>

                    <br/>

                    <button
                        id="submit"
                        node_ref=submit_btn_ref
                        type="submit"
                        class="submit_contact_form"
                        attr:disabled=move || !form_meets_reqs()
                    >
                        "Submit ➡"
                    </button>
                </fieldset>
            </Form>
        </aside>
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
struct Contact {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}

fn check_email_regex(email_addr: String) -> bool {
    // Regex for checking email addresses
    let email_regex: Regex =
        Regex::new(r"^[a-zA-Z0-9._%-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$").unwrap();

    // test email conforms
    email_regex.is_match(&email_addr)
}

fn check_phone_regex(phone: String) -> bool {
    let intl_phone_number_regex =
        Regex::new(r"^(\+?\d{0,3}|\d{0,4})[-.\\s]?([0-9]{3})[-.\\s]?([0-9]{3})[-.\\s]?([0-9]{4})$")
            .unwrap();

    // test phone number conforms to Intl standard
    intl_phone_number_regex.is_match(&phone)
}

fn check_form_meets_minimum_requirements(
    first_name_form_len: usize,
    last_name_form_len: usize,
    email_form_len: usize,
    phone_form_len: usize,
) -> bool {
    if first_name_form_len > 2 && last_name_form_len > 2 && phone_form_len > 6 && email_form_len > 7
    {
        true
    } else {
        false
    }
}
