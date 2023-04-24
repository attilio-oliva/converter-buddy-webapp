#![allow(non_snake_case)]
mod components;
use components::App;

fn main() {
    dioxus_web::launch(App);
}
