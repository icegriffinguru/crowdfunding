#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Crowdfunding {
    #[init]
    fn init(&self) {
    }
}