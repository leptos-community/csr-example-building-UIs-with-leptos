use leptos::*;

#[component]
pub fn Modal() -> impl IntoView {
    let (show_overlay, set_show_overlay) = create_signal(false);
    let (show_inside_overlay, set_show_inside_overlay) = create_signal(false);

    view! {
        <div>
            <button id="btn-show" on:click=move |_| set_show_overlay(true)>
                Show Modal
            </button>

            <Show when=show_overlay fallback=|| ()>
                <Portal mount=document().get_element_by_id("app").unwrap()>
                    <div class="portal_background">
                        <dialog class="portal_content" open=show_overlay>
                            <button id="btn-hide" on:click=move |_| set_show_overlay(false)>
                                "Close ❌"
                            </button>

                            <aside class="portal_body">

                                <p>"This is in the portal (modal's) body element"</p>

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
