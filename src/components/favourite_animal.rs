use leptos::*;
use std::str::FromStr;
use strum::*;


#[derive(
    Debug,
    Default,
    PartialEq,
    Eq,
    Clone,
    strum::Display,
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
pub fn SelectAnimal() -> impl IntoView {
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



        <p>
            "Your favourite animal is: " { move ||
                format!("{}", selected_option().to_string())
            }
        </p>

    }
}
