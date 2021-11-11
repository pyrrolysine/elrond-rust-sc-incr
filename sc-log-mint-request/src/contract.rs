#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Contract {

    #[init]
    fn init(
        &self,
        min_amount: Self::BigUint
    ) -> SCResult<()> {
        self.min_bid().set(&min_amount);
        Ok(())
    }

    #[endpoint]
    #[only_owner]
    fn claim(&self) -> () {
        self.send().direct_egld(
            &self.blockchain().get_owner_address(),
            &self.blockchain().get_sc_balance(&TokenIdentifier::egld(), 0),
            &[]
        );
    }


    #[endpoint]
    fn count(&self) -> usize {
        self.tokens().len()
    }

    #[endpoint]
    fn get_min_bid(&self) -> Self::BigUint {
        self.min_bid().get()
    }


    #[endpoint]
    #[payable("EGLD")]
    fn mint(
        &self,
        #[payment_amount] amount: Self::BigUint,
        lon: u64,
        lat: u64,
    ) -> bool {
        if amount >= self.min_bid().get() {
            let latlon = (lon, lat);
            self.minters().push(&self.blockchain().get_caller());
            self.tokens().push(&latlon);
            true
        } else {
            false
        }
    }

    #[endpoint]
    fn get(
        &self,
        index: usize
    ) -> Option<(Address, (u64, u64))> {
        if index <= self.count() + 1 && self.count() != 0 {
            Some((self.minters().get(index), self.tokens().get(index)))
        } else {
            None
        }
    }


    #[storage_mapper("data_min_bid")]
    fn min_bid(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[storage_mapper("data_addrs")]
    fn minters(&self) -> VecMapper<Self::Storage, Address>;

    #[storage_mapper("data_tokens")]
    fn tokens(&self) -> VecMapper<Self::Storage, (u64, u64)>;

}
