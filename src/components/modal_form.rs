// use cfg_if::cfg_if;
use leptos::html::Div;
use leptos::*;

use wasm_bindgen::closure::Closure;

use leptos_use::on_click_outside;

use regex_lite::Regex;
use wasm_bindgen::JsCast;


#[component]
pub fn FormModal() -> impl IntoView {
    // --- Set Up Modal (with click outside to dismiss behaviour) ---
    let (show_overlay, set_show_overlay) = create_signal(false);

    // add event listener to window & close modal when clicked outside modal window
    let modal_target: NodeRef<Div> = create_node_ref::<Div>();
    on_cleanup(on_click_outside(modal_target, move |_| {
        set_show_overlay(false)
    }));

    // handle keyboard "Escape" key press to dismiss modal
    let window = web_sys::window().unwrap();

    let handle_click = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
        if event.location() == 27 {
            logging::log!("escape key was pressed!")
        }
    }) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("keydown", handle_click.as_ref().unchecked_ref())
        .expect("problem window evt listener");

    // --- End Modal Setup ---


    view! {
        <button id="btn-show" on:click=move |_| set_show_overlay(true)>
            "Show Form Modal"
        </button>

        <Show when=show_overlay fallback=|| ()>
            <Portal mount=document().get_element_by_id("app").unwrap()>
                <div class="portal_background">
                    <div _ref=modal_target>
                        <dialog class="portal_content" open=show_overlay>

                            <div>
                                <button id="btn-hide" on:click=move |_| set_show_overlay(false)>
                                    "Close ❌"
                                </button>

                                <ModalBody set_show_overlay/>

                            </div>

                        </dialog>
                    </div>
                </div>
            </Portal>
        </Show>
    }
}

#[component]
fn ModalBody(set_show_overlay: WriteSignal<bool>) -> impl IntoView {
    // --- Modal Body ---
    let (show_inside_overlay, set_show_inside_overlay) = create_signal(false);


    // --- email address form ---
    let (email_addr, set_email_addr) = create_signal("".to_string());
    // Update email addr field
    let on_input = move |evt| {
        set_email_addr(event_target_value(&evt));
    };

    // Regex for checking email addresses
    let regex = move || Regex::new(r"^[A-Z0-9._%-]+@[A-Z0-9.-]+\.[A-Z]{2,4}$").unwrap();


    // let input: &'static str = format!("{}", email_addr()).as_str();

    let email_is_good = move || regex().is_match(email_addr().as_str());

    let (check_email, _set_check_email) = create_signal(email_is_good());

    view! {
        <aside class="portal_body">

            <p>"This is in the modal's (portal) body element"</p>

            <button
                id="btn-toggle"
                on:click=move |_| set_show_inside_overlay(!show_inside_overlay())
            >

                "Toggle modal content"
            </button>

            <Show when=show_inside_overlay fallback=|| view! { "Hidden" }>
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

                <span>
                    {move || {
                        if check_email() {
                            format!(" {}", "✅".to_string())
                        } else {
                            format!(" {}", "❌".to_string())
                        }
                    }}

                </span>

                <br/>
                <button on:click=move |_| set_show_overlay(false)>"Submit"</button>
            </form>
        </aside>
    }
}
