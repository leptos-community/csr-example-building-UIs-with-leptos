use cfg_if::cfg_if;
use leptos::html::Div;
use leptos::*;

cfg_if! {
    if #[cfg(not(feature = "ssr"))] {
        use wasm_bindgen::closure::Closure;
        use wasm_bindgen::JsCast;
    }
}


#[component]
pub fn Modal() -> impl IntoView {
    let (show_overlay, set_show_overlay) = create_signal(false);
    let (show_inside_overlay, set_show_inside_overlay) = create_signal(false);

    // add event listener to window
    let window = web_sys::window().unwrap();

    // add evt listener to window
    let dialog_ref: NodeRef<Div> = create_node_ref::<Div>();

    let handle_click = Closure::wrap(Box::new(move |evt: web_sys::PointerEvent| {
        let node = dialog_ref
            .get_untracked()
            .expect("input ref should be loaded");
        if let Some(target) = evt.target() {
            let target_node = target.dyn_into::<web_sys::Node>().ok();
            let is_inside = node.contains(target_node.as_ref());
            if !is_inside {
                set_show_overlay(false);
            }
        }
    }) as Box<dyn FnMut(_)>);

    window
        .add_event_listener_with_callback("click", handle_click.as_ref().unchecked_ref())
        .expect("problem adding modal evt listener");

    handle_click.forget();

    view! {
        <div _ref=dialog_ref>
            <button id="btn-show" on:click=move |_| set_show_overlay(true)>
                Show Modal
            </button>

            <Show when=show_overlay fallback=|| ()>
                <Portal mount=document().get_element_by_id("app").unwrap()>
                    <div class="portal_background">
                        <dialog class="portal_content" open=show_overlay id="dialog">

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
                </Portal>
            </Show>
        </div>
    }
}
