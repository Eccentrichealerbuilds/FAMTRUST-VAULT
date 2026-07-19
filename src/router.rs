use crate::contract_function_page::ContractFunctionPage;
use crate::{
    app::Home, contract_interact::InteractWithContract, pages::import_wallet::ImportWallet,
};
use dioxus::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum MyRoute {
    #[route("/")]
    Home,
    #[route("/import-wallet")]
    ImportWallet {},
    #[route("/contract-interact")]
    InteractWithContract,
    #[route("/function/:func")]
    ContractFunctionPage { func: String },
}
