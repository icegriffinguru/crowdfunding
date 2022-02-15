#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(TopEncode, TopDecode, TypeAbi, PartialEq, Clone, Copy)]
pub enum Status {
    FundingPeriod,
    Successful,
    Failed,
}

#[elrond_wasm::contract]
pub trait Crowdfunding {
    #[endpoint]
    #[payable("*")]
    fn fund(
        &self,
        #[payment_amount] payment: BigUint
    ) -> SCResult<()> {
        let current_time = self.blockchain().get_block_nonce();
        require!(current_time < self.deadline().get(), "cannot fund after deadline");

        let caller = self.blockchain().get_caller();
        self.deposit(&caller).update(|deposit| *deposit += payment);

        Ok(())
    }

    #[view]
    fn status(&self) -> Status {
        if self.blockchain().get_block_nonce() < self.deadline().get() {
            return Status::FundingPeriod;
        } else if self.get_current_funds() >= self.target().get() {
            return Status::Successful;
        } else {
            return Status::Failed;
        }
    }

    #[view(getCurrentFunds)]
    fn get_current_funds(&self) -> BigUint {
        return self.blockchain().get_sc_balance(&TokenIdentifier::egld(), 0);
    }

    #[endpoint]
    fn claim(&self) -> SCResult<()> {
        match self.status() {
            Status::FundingPeriod => sc_error!("cannot claim before deadline"),
            Status::Successful => {
                let caller = self.blockchain().get_caller();
                require!(
                    caller == self.blockchain().get_owner_address(),
                    "only owner can claim successful funding"
                );

                let sc_balance = self.get_current_funds();
                self.send().direct(&caller, &TokenIdentifier::egld(), 0, &sc_balance, b"claim");

                Ok(())
            },
            Status::Failed => {
                let caller = self.blockchain().get_caller();
                let deposit = self.deposit(&caller).get();

                if deposit > 0u32 {
                    self.deposit(&caller).clear();
                    self.send().direct(&caller, &TokenIdentifier::egld(), 0, &deposit, b"claim");
                }

                Ok(())
            }
        }
    }

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