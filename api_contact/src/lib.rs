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

    println!("Origin header: {:?}", req.header("Origin"));


    let request_body = req.body();

    let req_str = str::from_utf8(&request_body).unwrap();


    let first_name = req_str
        .lines()
        .into_iter()
        .nth(3)
        .unwrap_or_else(|| "Error")
        .to_string();

    let last_name = req_str
        .lines()
        .into_iter()
        .nth(7)
        .unwrap_or_else(|| "McError")
        .to_string();

    let email = req_str
        .lines()
        .into_iter()
        .nth(11)
        .unwrap_or_else(|| "error@example.com")
        .to_string();

    let mut phone = req_str
        .lines()
        .into_iter()
        .nth(15)
        .unwrap_or_else(|| "250-555-5555")
        .to_string();

    let phone: String = if !phone.contains('-') && !phone.contains('+') {
        let phone = match phone.len() {
            10 => {
                phone.insert(3, '-');
                phone.insert(7, '-');
                phone
            }
            11 => {
                phone.insert(1, '-');
                phone.insert(5, '-');
                phone.insert(9, '-');
                phone
            }
            12 => {
                phone.insert(2, '-');
                phone.insert(5, '-');
                phone.insert(10, '-');
                phone
            }
            13 => {
                phone.insert(2, '-');
                phone.insert(6, '-');
                phone.insert(11, '-');
                phone
            }
            _ => phone,
        };
        phone
    } else {
        phone
    };

    let contact = Contact {
        first_name,
        last_name,
        email,
        phone,
    };

    let contact_json = serde_json::to_string(&contact)?;

    Ok(Response::builder()
        .status(202)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "POST")
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
