pub mod home;

use dioxus::prelude::*;
use crate::layouts::navbar::Navbar;
use home::Home;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
}
