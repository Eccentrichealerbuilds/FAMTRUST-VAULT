use dioxus::prelude::*;
pub mod android;
pub mod app;
pub mod contract;
pub mod contract_calls;
pub mod contract_function_page;
pub mod contract_functions;
pub mod contract_interact;
pub mod pages;
use crate::contract_functions::ContractFunctionsPanel;
mod router;
fn main() {
    dioxus::prelude::launch(app);
}
fn app() -> Element {
    rsx! {
        Router::<router::MyRoute> {}
    }
}
