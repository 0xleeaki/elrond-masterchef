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

    /// @notice Add a new LP to the pool. Can only be called by the owner.
    /// DO NOT add the same LP token more than once. Rewards will be messed up if you do.
    /// @param alloc_point AP of the new pool.
    /// @param lp_token Address of the LP token.
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
        // emit LogPoolAddition(new_pool_id, allocPoint, lp_token);

        Ok(())
    }

    /// @notice Update the given pool's reward allocation point. Can only be called by the owner.
    /// @param pool_id The index of the pool. See `poolInfo`.
    /// @param alloc_point New AP of the pool.
    #[only_owner]
    #[endpoint]
    fn set(&self, pool_id: u64, alloc_point: BigUint) -> SCResult<()> {
        require!(!self.pool_info(pool_id).is_empty(), "Pool is not exist");
        let mut pool_info = self.pool_info(pool_id).get();
        self.total_alloc_point().update(|total_alloc_point| {
            *total_alloc_point += (&alloc_point - &pool_info.alloc_point)
        });
        pool_info.alloc_point = alloc_point;
        self.pool_info(pool_id).set(&pool_info);
        // emit LogSetPool(pool_id, alloc_point);

        Ok(())
    }

    /// @notice Sets the reward per second to be distributed. Can only be called by the owner.
    /// @param reward_per_second The amount of reward to be distributed per second.
    #[only_owner]
    #[endpoint]
    fn set_reward_per_second(&self, reward_per_second: BigUint) -> SCResult<()> {
        self.reward_per_second().set(&reward_per_second);
        // emit LogRewardPerSecondChanged(reward_per_second);

        Ok(())
    }

    /// @notice Set the new fund contract.
    /// @param fund The address of new fund contract.
    #[only_owner]
    #[endpoint]
    fn set_fund(&self, fund: ManagedAddress) -> SCResult<()> {
        self.fund().set(&fund);
        // emit LogFundChanged(fund);

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
