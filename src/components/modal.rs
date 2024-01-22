// use cfg_if::cfg_if;
use std::convert::Into;

use leptos::html::Div;
use leptos::*;

use wasm_bindgen::closure::Closure;

use leptos_use::on_click_outside;
use web_sys::KeyboardEvent;


async fn noop(ev: KeyboardEvent) -> String {
    logging::log!("You tried to escape the modal!");
    logging::log!("evt: {:?}", ev);
    "it worked".to_string()
}

#[component]
pub fn Modal() -> impl IntoView {
    // --- Set Up Modal (with 'click outside modal to dismiss' behaviour) ---
    let (show_modal, set_show_modal) = create_signal(false);


    // Dismiss modal when "Escape" (or 'q') key is pressed
    let dismiss_modal_with_keyboard = window_event_listener(ev::keypress, move |ev| {
        if ev.key() == "Escape" || ev.key() == "q" || ev.key() == "Q" {
            logging::log!("You tried to escape the modal!");
            set_show_modal(false);
        }

        logging::log!("ev.key: {}", ev.key());
        logging::log!("ev.code: {}", ev.code());
        logging::log!("ev.key_code: {}", ev.key_code());
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
            Show Modal
        </button>

        <Show when=show_modal fallback=|| ()>
            <Portal mount=document().get_element_by_id("app").unwrap()>
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
    let (show_inside_overlay, set_show_inside_overlay) = create_signal(false);
    // --- End Modal Body ---
    view! {
        <aside class="modal_body">

            <p>"This is in the modal's (portal) body"</p>

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
        </aside>
    }
}


// Option 1: handle keyboard "Escape" key press to dismiss modal
// let window = web_sys::window().unwrap();

// let handle_click = Closure::wrap(Box::new(move |event: web_sys::KeyboardEvent| {
//     if event.key() == "Escape" {
//         logging::log!("escape key was pressed!")
//     }
// }) as Box<dyn FnMut(_)>);

// let handle_click = move |evt: web_sys::KeyboardEvent| {
//     logging::log!("ev.key: {}", evt.key());
//     if evt.key() == "Escape" {
//         logging::log!("escape key was pressed!")
//     }
// };

// window
//     .add_event_listener_with_callback("keydown", handle_click(event_target(evt)))
//     .expect("problem window evt listener");

// Option 2: handle keyboard "Escape" keypress to dismiss modal
