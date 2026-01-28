use dioxus::prelude::*;
use crate::components::test::HelloWorld;

#[component]
pub fn Home() -> Element {
    rsx! {
        HelloWorld {}
    }
}
