#![no_std]
#![allow(unused_attributes)]

elrond_wasm::imports!();

#[elrond_wasm_derive::contract(PingPongImpl)]
pub trait PingPong {
	#[init]
	fn init(
		&self,
	) {
	}

	#[endpoint]
    fn tick(&self) -> i64 {
        self.blockchain().get_block_timestamp()
    }
}
