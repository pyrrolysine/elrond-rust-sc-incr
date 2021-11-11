#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Contract {

    #[init]
    fn init(&self) -> SCResult<()> {
        Ok(())
    }


    #[endpoint]
    fn tick(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

}
