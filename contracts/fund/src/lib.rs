#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Fund {
    #[init]
    fn init(&self, reward: TokenIdentifier, chef: ManagedAddress) -> SCResult<()> {
        require!(
            reward.is_egld() || reward.is_valid_esdt_identifier(),
            "Invalid reward token"
        );
        require!(
            self.blockchain().is_smart_contract(&chef),
            "The chef address is not a smart contract"
        );

        self.reward().set(&reward);
        self.chef().set(&chef);
        Ok(())
    }

    /* ========== PUBLIC FUNCTIONS ========== */

    #[endpoint]
    fn transfer(&self, to: ManagedAddress, amount: BigUint) -> SCResult<()> {
        require!(
            &self.blockchain().get_caller() == &self.chef().get(),
            "Only masterchef can call this function"
        );
        let reward = self.reward().get();
        let balance = self.get_current_funds();
        require!(amount > BigUint::from(0u32), "Invalid amount");
        require!(&amount <= &balance, "> balance");

        // Interactions
        self.send().direct(&to, &reward, 0, &amount, &[]);

        Ok(())
    }

    #[only_owner]
    #[endpoint]
    fn rescue_fund(&self, amount: BigUint) -> SCResult<()> {
        let reward = self.reward().get();
        let balance = self.get_current_funds();
        require!(amount > BigUint::from(0u32), "Invalid amount");
        require!(&amount <= &balance, "> balance");

        // Interactions
        self.send()
            .direct(&self.blockchain().get_caller(), &reward, 0, &amount, &[]);

        Ok(())
    }
    /* ========== VIEW FUNCTION ========== */

    #[view(getCurrentFunds)]
    fn get_current_funds(&self) -> BigUint {
        let token = self.reward().get();
        self.blockchain().get_sc_balance(&token, 0)
    }

    /* ========== STORAGE ========== */

    #[view(getReward)]
    #[storage_mapper("reward")]
    fn reward(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getChef)]
    #[storage_mapper("chef")]
    fn chef(&self) -> SingleValueMapper<ManagedAddress>;
}
