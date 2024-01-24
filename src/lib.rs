use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use leptos_use::use_preferred_dark;

// Modules
mod components;
mod pages;

// Top-Level pages
use crate::pages::contact::Contact;
use crate::pages::examples::Examples;
use crate::pages::home::Home;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    // Set HTML's `attr:data-theme` to user preference
    let is_dark_preferred = use_preferred_dark();
    let current_theme = move || {
        if is_dark_preferred() {
            format!("{}", "dark".to_string())
        } else {
            format!("{}", "light".to_string())
        }
    };

    view! {
        <div id="root">
            <Html lang="en" dir="ltr" attr:data-theme=current_theme/>

            // sets the document title
            <Title text="Welcome to Leptos CSR"/>

            // injects metadata in the <head> of the page
            <Meta charset="UTF-8"/>
            <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

            <Meta
                name="description"
                content="Examples of using the Leptos Fullstack Rust Framework to build web pages and web apps"
            />

            <ErrorBoundary fallback=|errors| {
                view! {
                    <h1>"Uh oh! Something went wrong!"</h1>

                    <p>"Errors: "</p>
                    // Render a list of errors as strings - good for development purposes
                    <ul>
                        {move || {
                            errors
                                .get()
                                .into_iter()
                                .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                                .collect_view()
                        }}

                    </ul>
                }
            }>

                <Router>
                    <div class="nav_style">
                        <nav class="main_nav">
                            <A href="">
                                <img
                                    class="logo"
                                    src="leptos_logo.png"
                                    alt="A logo for the Leptos fullstack Rust web framework"
                                    height="75"
                                    width="75"
                                />
                            </A>

                            <menu class="menu">
                                <A href="">"Home"</A>

                                <A href="examples">"Examples"</A>

                                <A href="contact">"Contact"</A>
                            </menu>

                        </nav>
                    </div>

                    <main class="page_container">
                        <Routes>
                            <Route path="/" view=Home/>
                            <Route path="/examples" view=Examples/>
                            <Route path="/contact" view=Contact/>

                            <Route path="/*any" view=NotFound/>
                        </Routes>
                    </main>
                </Router>

            </ErrorBoundary>
        </div>

        // When no z-index is set on elements, elements lower in the hierarchy take higher precedence; so appending the portal root *last* ensures the portal container element will have higher z-index precedence
        <div id="portal_root"></div>
    }
}
