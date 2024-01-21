use leptos::*;
use std::str::FromStr;
use strum::*;

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    view! {
        <div class="container">

            <picture>
                <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_pref_dark_RGB.svg" media="(prefers-color-scheme: dark)" />
                <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo" height="200" width="400"/>
            </picture>

            <h1>"Welcome to Leptos"</h1>

            <div class="buttons">
                <Button />
                <Button increment=5 />
            </div>

            <SelectAnimals />

        </div>
    }
}

/// A parameterized incrementing button
#[component]
fn Button(#[prop(default = 1)] increment: i32) -> impl IntoView {
    let (count, set_count) = create_signal(0);
    view! {
        <button
            on:click= move |_| {
                set_count(count() + increment)
            }
        >
            "Click me: " {count}
        </button>
    }
}

#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    Clone,
    strum::Display,
    strum::EnumIs,
    strum::EnumIter,
    strum::EnumString,
    strum::IntoStaticStr,
)]
enum Animals {
    #[default]
    Other,
    Dog,
    Cat,
    Rabbit,
    Bird,
    Snake,
}

#[component]
fn SelectAnimals() -> impl IntoView {
    let (selected_option, set_selected_option) = create_signal(Animals::default());

    let options = Animals::iter()
        .map(|itm| {
            view! { <option value=itm.to_string()>{itm.to_string()}</option> }
        })
        .collect_view();

    let on_input = move |evt| {
        let value = event_target_value(&evt);
        let value = value.as_ref();

        let parsed = FromStr::from_str(value).unwrap_or_default();

        set_selected_option(parsed)
    };

    let selected = move || selected_option().to_string();

    let on_change = move |_| logging::log!("You selected: {}", selected_option());

    view! {
        <hr/>
        <h2>
            <label for="favourite_animal">"Select Your Favourite Animal"</label>
        </h2>
        <select
            name="animals"
            id="favourite_animal"
            title="Favourite Animal"
            on:input=on_input
            prop:selected=selected
            on:change=on_change
        >
            {options}
        </select>
    }
}
