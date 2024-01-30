// use anyhow::Error;

use serde::{Deserialize, Serialize};
use serde_json;

use serde_qs;
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

    let contact = match serde_qs::from_bytes::<Contact>(request_body) {
        Ok(contact) => {
            println!("{:?}", contact);
            contact
        }
        Err(e) => {
            println!("Err: {}", e);
            Contact {
                first_name: "".to_string(),
                last_name: "".to_string(),
                email: "".to_string(),
                phone: "".to_string(),
            }
        }
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
