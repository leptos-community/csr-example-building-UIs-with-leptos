use cfg_if::cfg_if;
use leptos::html::Div;
// use leptos_use::use_window;
// use leptos_use::UseDocument;

// use std::convert::From;
use std::convert::Into;

use leptos::*;

use leptos_use::on_click_outside;


// use leptos_use::use_window;


cfg_if! {
    if #[cfg(not(feature = "ssr"))] {
        // use wasm_bindgen::closure::Closure;
        // use wasm_bindgen::JsCast;
    }
}


#[component]
pub fn Modal() -> impl IntoView {
    // --- Set Up Modal (with click outside to dismiss behaviour) ---
    let (show_overlay, set_show_overlay) = create_signal(false);


    // add event listener to window & close modal when clicked outside modal window
    cfg_if! {
        if #[cfg(not(feature = "ssr"))] {

        // let window = web_sys::window().unwrap();
        // let window = use_window();
        window_event_listener(ev::keypress, |ev| {

            // logging::log!("{}", ev.code());
            // if keycode == "escape".to_string() {
            //     set_show_overlay(false);
            // }
            if ev.code() == "Escape".to_string().as_str() {
                // &set_show_overlay(false);
                logging::log!("{}", ev.code());
            }

        });

        // use `Leptos-Use` to setup modal 'click outside modal to dismiss' behaviour
        let modal_target: NodeRef<Div> = create_node_ref::<Div>();
        on_cleanup(on_click_outside(modal_target, move |_| {
            set_show_overlay(false)
        }));
    }}

    // --- Modal Body ---
    let (show_inside_overlay, set_show_inside_overlay) = create_signal(false);
    // --- Modal Body ---


    view! {
        <button id="btn-show" on:click=move |_| set_show_overlay(true)>
            Show Modal
        </button>

        <Show when=show_overlay fallback=|| ()>
            <Portal mount=document().get_element_by_id("app").unwrap()>
                <div class="portal_background">
                    <div node_ref=modal_target>
                        <dialog class="portal_content" open=show_overlay>

                            <button id="btn-hide" on:click=move |_| set_show_overlay(false)>
                                "Close ❌"
                            </button>

                            <aside class="portal_body">

                                <p>"This is in the modal's (portal) body element"</p>

                                <button
                                    id="btn-toggle"
                                    on:click=move |_| set_show_inside_overlay(
                                        !show_inside_overlay(),
                                    )
                                >

                                    "Toggle modal content"
                                </button>

                                <Show when=show_inside_overlay fallback=|| view! { "Hidden" }>
                                    "Visible"
                                </Show>

                                <p>"More contents..."</p>
                            </aside>

                        </dialog>
                    </div>
                </div>
            </Portal>
        </Show>
    }
}
