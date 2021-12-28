#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod pool_info;
mod user_info;

use pool_info::PoolInfo;
use user_info::UserInfo;

#[elrond_wasm::contract]
pub trait MasterChef {
    /* ========== INIT FUNCTION ========== */

    #[init]
    fn init(&self, fund: ManagedAddress, reward: TokenIdentifier) -> SCResult<()> {
        require!(
            reward.is_egld() || reward.is_valid_esdt_identifier(),
            "Invalid reward token"
        );
        require!(
            self.blockchain().is_smart_contract(&fund),
            "The fund address is not a smart contract"
        );
        self.reward().set(&reward);
        self.fund().set(&fund);
        self.pool_length().set(&u64::MIN);
        Ok(())
    }

    /* ========== INTERNAL FUNCTIONS ========== */

    /* ========== RESTRICTED FUNCTIONS ========== */

    #[only_owner]
    #[endpoint]
    fn add(&self, alloc_point: BigUint, lp_token: TokenIdentifier) -> SCResult<()> {
        let pool_length = self.pool_length().get();
        for i in 1..=pool_length {
            let pool_info = self.pool_info(i).get();
            require!(pool_info.lp_token != lp_token, "Cannot add existing pool");
        }
        let new_pool_id = pool_length + 1;
        self.total_alloc_point()
            .update(|total_alloc_point| *total_alloc_point += &alloc_point);
        let last_reward_time = self.blockchain().get_block_timestamp();
        let new_pool = PoolInfo {
            lp_token,
            acc_reward_per_share: BigUint::zero(),
            last_reward_time,
            alloc_point,
        };
        self.pool_info(new_pool_id).set(&new_pool);
        // emit event: TODO

        Ok(())
    }

    /* ========== STORAGE ========== */

    #[view(getFund)]
    #[storage_mapper("fund")]
    fn fund(&self) -> SingleValueMapper<ManagedAddress>;

    #[view(getReward)]
    #[storage_mapper("reward")]
    fn reward(&self) -> SingleValueMapper<TokenIdentifier>;

    #[view(getTotalAllocPoint)]
    #[storage_mapper("totalAllocPoint")]
    fn total_alloc_point(&self) -> SingleValueMapper<BigUint>;

    #[view(getRewardPerSecond)]
    #[storage_mapper("rewardPerSecond")]
    fn reward_per_second(&self) -> SingleValueMapper<BigUint>;

    #[view(getPoolLength)]
    #[storage_mapper("poolLength")]
    fn pool_length(&self) -> SingleValueMapper<u64>;

    #[view(getPoolInfo)]
    #[storage_mapper("poolInfo")]
    fn pool_info(&self, id: u64) -> SingleValueMapper<PoolInfo<Self::Api>>;

    #[view(getUserInfo)]
    #[storage_mapper("userInfo")]
    fn user_info(&self, user: &ManagedAddress) -> SingleValueMapper<UserInfo<Self::Api>>;
}
