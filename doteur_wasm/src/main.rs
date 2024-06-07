use doteur_wasm::App;
use leptos::{mount_to_body, view};

fn main() {
    mount_to_body(|| {
        view! {
            <App/>
        }
    });
}
