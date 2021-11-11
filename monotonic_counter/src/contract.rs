#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Contract {

    #[init]
    fn init(&self) -> SCResult<()> {
        self.counter().set(&0);
        Ok(())
    }


    #[endpoint]
    fn inc(&self) -> u64 {
        let mut value: u64 = self.counter().get();
        value += 1;
        self.counter().set(&value);
        value
    }

    #[endpoint]
    fn get(&self) -> u64 {
        self.counter().get()
    }

    #[storage_mapper("counter")]
    fn counter(&self) -> SingleValueMapper<Self::Storage, u64>;

}
