use leptos::*;

#[component]
pub fn Overlay() -> impl IntoView {
    let (show_overlay, set_show_overlay) = create_signal(false);
    let (show_inside_overlay, set_show_inside_overlay) = create_signal(false);

    view! {
        <div>
            <button id="btn-show" on:click=move |_| set_show_overlay(true)>
                Show Overlay
            </button>

            <Show when=show_overlay fallback=|| ()>
                <div>Show</div>
                <Portal mount=document().get_element_by_id("app").unwrap()>
                    <div class="portal_background">
                        <dialog class="portal_content" open=show_overlay>
                            <button id="btn-hide" on:click=move |_| set_show_overlay(false)>
                                "Close Overlay ❌"
                            </button>

                            <p>"This is in the portal (overlay's) body element"</p>

                            <button
                                id="btn-toggle"
                                on:click=move |_| set_show_inside_overlay(!show_inside_overlay())
                            >
                                "Toggle inner"
                            </button>

                            <Show when=show_inside_overlay fallback=|| view! { "Hidden" }>
                                "Visible"
                            </Show>
                        </dialog>
                    </div>
                </Portal>
            </Show>
        </div>
    }
}
