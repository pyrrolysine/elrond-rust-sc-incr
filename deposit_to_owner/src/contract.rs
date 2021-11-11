#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Contract {

    #[init]
    fn init(&self) -> SCResult<()> {
        self.owner().set(&self.blockchain().get_caller());
        Ok(())
    }

    #[payable("*")]
    #[endpoint]
    fn deposit(
        &self,
        #[payment_token] token: TokenIdentifier,
        #[payment_amount] amount: Self::BigUint
    ) -> Self::BigUint {
        let from = self.blockchain().get_caller();
        let old_amount = self.deposits(&from, &token).get();
        let new_amount = &old_amount + &amount;
        self.deposits(&from, &token).set(&new_amount);
        self.send().direct(&self.owner().get(), &token, 0, &amount, b"WIRING");
        new_amount
    }

    #[storage_mapper("deposits")]
    fn deposits(&self, address: &Address, token: &TokenIdentifier) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[storage_mapper("owner")]
    fn owner(&self) -> SingleValueMapper<Self::Storage, Address>;

}
