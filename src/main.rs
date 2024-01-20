use leptos::*;
use csr_tutorial::App;

fn main() {
    mount_to_body(|| {
        view! {
            <App />
        }
    })
}
