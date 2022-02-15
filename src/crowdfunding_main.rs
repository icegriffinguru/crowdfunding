#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Crowdfunding {
    #[view(getTarget)]
    #[storage_mapper("target")]
    fn target(&self) -> SingleValueMapper<BigUint>;

    #[view(getDeadline)]
    #[storage_mapper("deadline")]
    fn deadline(&self) -> SingleValueMapper<u64>;

    #[view(getDeposit)]
    #[storage_mapper("deposit")]
    fn deposit(&self, donor: &ManagedAddress) -> SingleValueMapper<BigUint>;

    #[init]
    fn init(&self, target: BigUint, deadline: u64) {
        self.target().set(&target);
        self.deadline(). set(&deadline);
    }
}