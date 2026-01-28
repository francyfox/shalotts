use dioxus::prelude::*;

#[component]
pub fn HelloWorld() -> Element {
    rsx! {
        div {
            p { "Hello world!" }
        }
    }
}
