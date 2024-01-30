use std::rc::Rc;

// use cfg_if::cfg_if;
use leptos::html::Div;
use leptos::*;
use leptos_router::{Form, FormProps};

use leptos_use::on_click_outside;

use regex_lite::Regex;
use web_sys::{Event, Response, SubmitEvent};


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


use serde::{Deserialize, Serialize};
use serde_json;

#[component]
fn ModalBody(set_show_modal: WriteSignal<bool>) -> impl IntoView {
    // --- Modal Body ---

    // Setup name fields
    let (first_name, set_first_name) = create_signal("".to_string());
    let (last_name, set_last_name) = create_signal("".to_string());

    let on_first_name_input = move |ev| {
        set_first_name(event_target_value(&ev));
    };
    let on_last_name_input = move |ev| {
        set_last_name(event_target_value(&ev));
    };

    let first_name_form_len = move || first_name().len();
    let last_name_form_len = move || last_name().len();


    // --- email address form, with email address checked by regex_lite ---
    let (email_addr, set_email_addr) = create_signal("".to_string());

    // Update email addr form field
    let on_email_input = move |ev| {
        set_email_addr(event_target_value(&ev));
    };

    // test if email addr conforms
    let check_email_resource =
        create_local_resource(
            email_addr,
            |email| async move { check_email_regex(email).await },
        );

    let is_email_good = move || check_email_resource.get().unwrap_or_else(|| false);

    let email_form_len = move || email_addr().len();


    // --- Telephone input, checked locally by regex_lite ---
    let (phone, set_phone) = create_signal("".to_string());

    let on_phone_input = move |ev| {
        set_phone(event_target_value(&ev));
    };

    // test if phone number conforms
    let check_phone_number_resource =
        create_local_resource(phone, |phone| async move { check_phone_regex(phone).await });

    let is_phone_number_good = move || check_phone_number_resource.get().unwrap_or_else(|| false);

    let phone_form_len = move || phone().len();


    // --- Submit Form ---
    // let post_form_data_action = create_action(|input: &(String, String, String, String)| async {

    // });


    // --- END email address form ---


    let navigate = leptos_router::use_navigate();

    // async move |data| logging::log!("{}", format!("{}", data))

    // let response = Rc::new(move || {

    // } as dyn Fn(&Response)());

    // navigate("/contact", Default::default());
    // set_show_modal(false);

    // let mut res: &Response =
    // let response = Rc::new(parse_response(res));

    // let submit_listener = move |ev: SubmitEvent| ev.prevent_default();

    // let form = document().get_element_by_id("contact_form").unwrap();

    // form.add_event_listener_with_callback("submit", submit_listener);

    let contact_form_ref = create_node_ref();

    let contact_ref = contact_form_ref.get().unwrap();

    let response = move || {};

    view! {
        <aside class="modal_body">

            <h2 class="modal_form_title">"Contact Us"</h2>

            <Form
                attr:id="contact_form"
                method="POST"
                action="http://localhost:3000/api/contact"
                class="contact_form"
                on_response=response
                node_ref=contact_form_ref
            >

                <fieldset class="contact_form_fieldset" form="contact_form" name="contact_form">

                    <div class="contact_input">
                        <label class="input_label" for="first_name">
                            "First name:"
                        </label>
                        <input
                            class="contact_input"
                            autofocus
                            name="first_name"
                            id="first_name"
                            type="text"
                            placeholder="Johnny"
                            prop:value=first_name
                            on:input=on_first_name_input
                        />
                        <Transition fallback=|| view! { format!("{}", "🤔".to_string()) }>
                            <span>
                                {move || {
                                    if first_name_form_len() == 0 {
                                        format!(" {}", "*".to_string())
                                    } else if first_name_form_len() > 2 {
                                        format!(" {}", "✅".to_string())
                                    } else {
                                        format!(" {}", "❌".to_string())
                                    }
                                }}

                            </span>
                        </Transition>
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
                        <Transition fallback=|| view! { format!("{}", "🤔".to_string()) }>
                            <span>
                                {move || {
                                    if last_name_form_len() == 0 {
                                        format!(" {}", "*".to_string())
                                    } else if last_name_form_len() > 2 {
                                        format!(" {}", "✅".to_string())
                                    } else {
                                        format!(" {}", "❌".to_string())
                                    }
                                }}

                            </span>
                        </Transition>
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
                        <Transition fallback=|| view! { format!("{}", "🤔".to_string()) }>
                            <span>
                                {move || {
                                    if email_form_len() == 0 {
                                        format!(" {}", "*".to_string())
                                    } else if is_email_good() {
                                        format!(" {}", "✅".to_string())
                                    } else {
                                        format!(" {}", "❌".to_string())
                                    }
                                }}

                            </span>
                        </Transition>
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
                        <Transition fallback=|| view! { format!("{}", "🤔".to_string()) }>
                            <span>
                                {move || {
                                    if phone_form_len() == 0 {
                                        format!(" {}", "*".to_string())
                                    } else if is_phone_number_good() {
                                        format!(" {}", "✅".to_string())
                                    } else {
                                        format!(" {}", "❌".to_string())
                                    }
                                }}

                            </span>
                        </Transition>
                    </div>

                    <br/>
                    // type="submit"
                    <button type="submit" class="submit_contact_form" on:click=move |_ev| { ev }>

                        // navigate("/contact", Default::default());
                        // set_show_modal(false);

                        "Submit ➡"
                    </button>
                </fieldset>
            </Form>
        </aside>
    }
}


async fn check_email_regex(email_addr: String) -> bool {
    // Regex for checking email addresses
    let email_regex: Regex =
        Regex::new(r"^[a-zA-Z0-9._%-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,4}$").unwrap();

    // test email conforms
    email_regex.is_match(&email_addr)
}

async fn check_phone_regex(phone: String) -> bool {
    let intl_phone_number_regex =
        Regex::new(r"^(\+?\d{0,3}|\d{0,4})[-.\\s]?([0-9]{3})[-.\\s]?([0-9]{3})[-.\\s]?([0-9]{4})$")
            .unwrap();

    // test phone number conforms to Intl standard
    intl_phone_number_regex.is_match(&phone)
}

fn parse_response(response: &Response) {
    let res = response.json().unwrap();
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
struct Contact {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}
