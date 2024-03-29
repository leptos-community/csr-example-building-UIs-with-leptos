use std::convert::Into;

use leptos::html::Div;
use leptos::*;

use leptos_use::on_click_outside;


#[component]
pub fn Modal() -> impl IntoView {
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
            Show Modal
        </button>

        <Show when=show_modal fallback=|| ()>
            <Portal mount=document().get_element_by_id("portal_root").unwrap()>
                <div class="modal_background">
                    <div node_ref=modal_target>
                        <dialog class="modal_content" open=show_modal>

                            <button id="btn-hide" on:click=move |_| set_show_modal(false)>
                                "Close ❌"
                            </button>

                            <ModalBody/>

                        </dialog>
                    </div>
                </div>
            </Portal>
        </Show>
    }
}

#[component]
fn ModalBody() -> impl IntoView {
    // --- Modal Body ---
    let (contents_hidden, set_contents_hidden) = create_signal(false);
    // --- End Modal Body ---
    view! {
        <aside class="modal_body">

            <p>"This is in the modal's (portal) body"</p>

            <button id="btn-toggle" on:click=move |_| set_contents_hidden(!contents_hidden())>

                "Toggle modal content"
            </button>

            <Show when=contents_hidden fallback=|| view! { "Hidden" }>
                "Visible"
            </Show>

            <p>"More contents..."</p>
        </aside>
    }
}


// Other Option: handle keyboard "Escape" key press to dismiss modal
// let window = web_sys::window().unwrap();

// let handle_click = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
//     if event.key() == "Escape" {
//         set_show_modal(false);
//     }
// }) as Box<dyn FnMut(_)>);

// let handle_click = move |ev: web_sys::KeyboardEvent| {
//     if ev.key() == "Escape" {
//         set_show_modal(false);
//     }
// };

// window
//     .add_event_listener_with_callback("keydown", handle_click(...))
//     .expect("problem window evt listener");
