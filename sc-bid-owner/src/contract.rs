
/*
 * Single-Auction Contract
 *
 * Handles a single NFT at a time, but can be reused after an auction ends.
 */

#![no_std]

elrond_wasm::imports!();

#[elrond_wasm::contract]
pub trait Contract {

    #[init]
    fn init(&self) -> SCResult<()> {
        self.auction_started().set(&false);
        Ok(())
    }

    #[endpoint]
    fn clock(&self) -> u64 {
        self.now()
    }

    #[endpoint]
    fn status(&self) -> bool {
        self.auction_started().get()
    }

    #[endpoint]
    fn max_epoch(&self) -> u64 {
        self.expiration().get()
    }

    #[endpoint]
    fn get_top_bid(&self) -> Self::BigUint {
        self.price().get()
    }

    #[endpoint]
    fn nft(&self) -> (TokenIdentifier, u64) {
        (self.token_id().get(), self.token_nonce().get())
    }

    #[endpoint]
    #[payable("*")]
    #[only_owner]
    fn auction(
        &self,
        #[payment_token] token: TokenIdentifier,
        #[payment_nonce] nonce: u64,
        #[payment_amount] amount: Self::BigUint,
        initial_price: Self::BigUint,
        expiration: u64
    ) -> bool {
        if nonce != 0 && amount == 1 && expiration > self.now() {
            // work only with NFTs

            self.token_id().set(&token);
            self.token_nonce().set(&nonce);
            self.price().set(&initial_price);
            self.min_price().set(&initial_price);
            self.expiration().set(&expiration);
            self.auction_started().set(&true);

            true
        } else {
            false
        }
    }

    #[endpoint]
    #[payable("EGLD")]
    fn bid(
        &self,
        #[payment_amount] amount: Self::BigUint
    ) -> bool {
        let caller = self.blockchain().get_caller();
        let owner = self.blockchain().get_owner_address();
        if amount > self.price().get() && caller != owner && self.auction_started().get() {
            // register bidder
            self.update_bid(caller, amount);

            true
        } else {
            // value below bid, refund fraction

            self.send().direct_egld(
                &caller,
                &(amount / Self::BigUint::from_bytes_be(&[2])),
                &[]
            );

            false
        }
    }

    #[endpoint]
    fn unbid(&self) -> () {
        self.update_bid(self.blockchain().get_caller(), Self::BigUint::zero())
    }

    #[endpoint]
    #[only_owner]
    fn cancel(&self) -> bool {
        if self.auction_started().get() {
            for k in 1..(self.bidders().len() + 1) {
                let addr = self.bidders().get(k);
                self.update_bid(addr, Self::BigUint::zero())
            }

            self.send().direct(
                &self.blockchain().get_owner_address(),
                &self.token_id().get(),
                self.token_nonce().get(),
                &Self::BigUint::from_bytes_be(&[1]),
                &[]
            );

            self.auction_started().set(&false);

            true
        } else {
            false
        }
    }

    #[endpoint]
    #[only_owner]
    fn accept(&self) -> bool {
        if  self.auction_started().get()
            && self.expiration().get() < self.now()
            && self.bidders().len() != 0
        {
            let mut max_k: usize = 0;
            let mut winner_amount: Self::BigUint = Self::BigUint::zero();

            for k in 1..(self.bidders().len() + 1) {
                let amount = self.amounts().get(k);
                if amount > winner_amount {
                    winner_amount = amount;
                    max_k = k
                }
            }
            if winner_amount >= self.min_price().get() {

                for k in 1..(self.bidders().len() + 1) {
                    let amount = self.amounts().get(k);
                    if amount != 0 && max_k != k {
                        self.send().direct_egld(
                            &self.bidders().get(k),
                            &amount,
                            &[]
                        )
                    }
                }

                self.send().direct(
                    &self.bidders().get(max_k),
                    &self.token_id().get(),
                    self.token_nonce().get(),
                    &Self::BigUint::from_bytes_be(&[1]),
                    &[]
                );

                self.send().direct_egld(
                    &self.blockchain().get_owner_address(),
                    &winner_amount,
                    &[]
                );

                self.auction_started().set(&false);

                self.bidders().clear();
                self.amounts().clear();

                true
            } else {
                false
            }
        } else {
            false
        }
    }

    #[storage_mapper("token_id")]
    fn token_id(&self) -> SingleValueMapper<Self::Storage, TokenIdentifier>;
    
    #[storage_mapper("token_nonce")]
    fn token_nonce(&self) -> SingleValueMapper<Self::Storage, u64>;

    #[storage_mapper("bids_addrs")]
    fn bidders(&self) -> VecMapper<Self::Storage, Address>;

    #[storage_mapper("bids_amts")]
    fn amounts(&self) -> VecMapper<Self::Storage, Self::BigUint>;

    #[storage_mapper("bids_level")]
    fn price(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[storage_mapper("bids_level_init")]
    fn min_price(&self) -> SingleValueMapper<Self::Storage, Self::BigUint>;

    #[storage_mapper("auction_started")]
    fn auction_started(&self) -> SingleValueMapper<Self::Storage, bool>;

    #[storage_mapper("auction_expiration")]
    fn expiration(&self) -> SingleValueMapper<Self::Storage, u64>;


    fn now(&self) -> u64 {
        self.blockchain().get_block_timestamp()
    }

    fn update_bid(&self, bidder: Address, amount: Self::BigUint) -> () {
        let mut found: bool = false;

        // reset highest bid
        self.price().set(&self.min_price().get());

        for k in 1..(self.bidders().len() + 1) {
            if self.bidders().get(k) == bidder {
                found = true;

                // refund previous bid
                let old_amount = self.amounts().get(k);
                if old_amount != 0 {
                    self.send().direct_egld(&bidder, &old_amount, &[]);
                }

                // mutate to new bid
                self.amounts().set(k, &amount);

                // update highest bid
                if self.amounts().get(k) > self.price().get() {
                    self.price().set(&self.amounts().get(k))
                }
            }

        }

        if !found {
            // register new bidder
            self.bidders().push(&bidder);
            self.amounts().push(&amount);

            // update highest bid
            if amount > self.price().get() {
                self.price().set(&amount)
            }
        }
    }
}
