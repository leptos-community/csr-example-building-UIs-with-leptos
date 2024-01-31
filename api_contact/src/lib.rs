// use anyhow::Error;

use serde::{Deserialize, Serialize};
use serde_json;

use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

use std::str;


/// A simple Spin HTTP component.
#[http_component]
fn handle_api_contact(req: Request) -> anyhow::Result<impl IntoResponse> {
    println!(
        "Handling request.\n{:?}\nURL: {:?}",
        req.method(),
        req.header("spin-full-url")
    );

    let request_body = req.body();

    let req_str = str::from_utf8(&request_body).unwrap();


    let first_name = req_str
        .lines()
        .into_iter()
        .nth(3)
        .unwrap_or_else(|| "Error")
        .to_string();
    // println!("First name: {}", first_name);

    let last_name = req_str
        .lines()
        .into_iter()
        .nth(7)
        .unwrap_or_else(|| "McError")
        .to_string();
    // println!("Last name: {}", last_name);

    let email = req_str
        .lines()
        .into_iter()
        .nth(11)
        .unwrap_or_else(|| "error@example.com")
        .to_string();
    // println!("Email: {}", email);

    let phone = req_str
        .lines()
        .into_iter()
        .nth(15)
        .unwrap_or_else(|| "250-555-555")
        .to_string();
    // println!("Phone: {}", phone);

    let contact = Contact {
        first_name,
        last_name,
        email,
        phone,
    };


    let contact_json = serde_json::to_string(&contact)?;


    Ok(Response::builder()
        .status(202)
        .header("content-type", "text/json")
        .body(contact_json)
        .build())
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
struct Contact {
    first_name: String,
    last_name: String,
    email: String,
    phone: String,
}
