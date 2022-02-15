#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Crowdfunding {
    #[view]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, target: BigUint) {
        self.target().set(&target);
    }
}