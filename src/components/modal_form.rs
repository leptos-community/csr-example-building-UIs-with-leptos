// use cfg_if::cfg_if;
use leptos::html::Div;
use leptos::*;

use leptos_use::on_click_outside;

use regex_lite::Regex;


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
        <button id="btn-show" on:click=move |_| set_show_modal(true)>
            "Show Form Modal"
        </button>

        <Show when=show_modal fallback=|| ()>
            <Portal mount=document().get_element_by_id("app").unwrap()>
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
    let (contents_hidden, set_contents_hidden) = create_signal(false);


    // --- email address form, with email address checked by regex_lite ---
    let (email_addr, set_email_addr) = create_signal("".to_string());

    // Update email addr form field
    let on_input = move |evt| {
        set_email_addr(event_target_value(&evt));
    };

    // test if email addr conforms
    let check_email_resource =
        create_resource(
            email_addr,
            |val| async move { check_email_regex(val).await },
        );

    let is_email_good = move || check_email_resource.get().unwrap_or_else(|| false);

    // --- END email address form ---


    view! {
        <aside class="modal_body">

            <p>"This is in the modal's (portal) body element"</p>

            <button id="btn-toggle" on:click=move |_| set_contents_hidden(!contents_hidden())>

                "Toggle modal content"
            </button>

            <Show when=contents_hidden fallback=|| view! { "Hidden" }>
                "Visible"
            </Show>

            <p>"More contents..."</p>

            <form method="dialog">
                <label for="email">"Please enter your email address"</label>
                <br/>
                <input
                    name="email"
                    id="email"
                    type="email"
                    placeholder="your.email@website.com"
                    prop:value=email_addr
                    on:input=on_input
                />

                <Transition fallback=|| view! { format!("{}", "🤔".to_string()) }>

                    <span>
                        {move || {
                            if is_email_good() {
                                format!(" {}", "✅".to_string())
                            } else {
                                format!(" {}", "❌".to_string())
                            }
                        }}

                    </span>

                </Transition>

                <br/>
                <button on:click=move |_| set_show_modal(false)>"Submit"</button>
            </form>
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
