#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

mod pool_info;
mod user_info;

use pool_info::PoolInfo;
use user_info::UserInfo;

pub const ACC_REWARD_PRECISION: u64 = 10u64.pow(12);

#[elrond_wasm::contract]
pub trait MasterChef {
    /* ========== INIT FUNCTION ========== */

    #[init]
    fn init(&self, reward: TokenIdentifier) -> SCResult<()> {
        require!(
            reward.is_egld() || reward.is_valid_esdt_identifier(),
            "Invalid reward token"
        );
        // require!(
        //     self.blockchain().is_smart_contract(&fund),
        //     "The fund address is not a smart contract"
        // );

        self.reward().set(&reward);
        // self.fund().set(&fund);
        self.pool_length().set(&u64::MIN);
        Ok(())
    }

    /* ========== INTERNAL FUNCTIONS ========== */

    /// @notice Update reward variables of the given pool.
    /// @param pool_info
    fn update_pool(&self, pool_info: &mut PoolInfo<Self::Api>) {
        let block_timestamp = self.blockchain().get_block_timestamp();
        if (block_timestamp > pool_info.last_reward_time) {
            let lp_supply = self.blockchain().get_sc_balance(&pool_info.lp_token, 0);
            if (lp_supply > BigUint::from(0u32)) {
                let time: u64 = block_timestamp - &pool_info.last_reward_time;
                let reward_amount =
                    BigUint::from(time) * self.reward_per_second().get() * &pool_info.alloc_point
                        / self.total_alloc_point().get();
                pool_info.acc_reward_per_share += reward_amount * ACC_REWARD_PRECISION / lp_supply;
            }
            pool_info.last_reward_time = block_timestamp;
        }
    }

    /* ========== PUBLIC FUNCTIONS ========== */

    //TODO: mas_update_pools, withdraw_and_harvest, harvest_all_rewards

    /// @notice Deposit LP tokens to MCV2 for reward allocation.
    /// @param pool_id The index of the pool. See `pool_info`.
    /// @param to The receiver of `amount` deposit benefit.
    #[endpoint]
    #[payable("*")]
    fn deposit(
        &self,
        pool_id: u64,
        to: ManagedAddress,
        #[payment_token] token: TokenIdentifier,
        #[payment] amount: BigUint,
    ) -> SCResult<()> {
        require!(!self.pool_info(pool_id).is_empty(), "Pool is not exist");
        let pool_info = &mut self.pool_info(pool_id).get();
        require!(token == pool_info.lp_token, "Wrong token");
        let mut user_info = self.user_info(&to).get();

        // Effects
        self.update_pool(pool_info);
        user_info.amount += amount;
        user_info.reward_debt =
            &user_info.amount * &pool_info.acc_reward_per_share / ACC_REWARD_PRECISION;
        self.user_info(&to).set(&user_info);

        // emit Deposit(&self.blockchain().get_caller(), pool_id, amount, to);

        Ok(())
    }

    /// @notice Withdraw LP tokens from MasterChef.
    /// @param pool_id The index of the pool. See `pool_info`.
    /// @param amount LP token amount to withdraw.
    /// @param to Receiver of the LP tokens.
    #[endpoint]
    fn withdraw(&self, pool_id: u64, amount: BigUint, to: ManagedAddress) -> SCResult<()> {
        let sender = self.blockchain().get_caller();
        require!(!self.pool_info(pool_id).is_empty(), "Pool is not exist");
        let mut user_info = self.user_info(&sender).get();
        require!(user_info.amount >= amount, "withdraw: not good");
        let pool_info = &mut self.pool_info(pool_id).get();
        self.update_pool(pool_info);
        let accumulated_reward =
            &user_info.amount * &pool_info.acc_reward_per_share / ACC_REWARD_PRECISION;
        let pending_reward = &accumulated_reward - &user_info.reward_debt;

        // Effects
        user_info.amount = user_info.amount - &amount;
        user_info.reward_debt = accumulated_reward;
        self.user_info(&sender).set(&user_info);

        // Interactions
        self.send()
            .direct(&to, &self.reward().get(), 0, &pending_reward, &[]);
        self.send()
            .direct(&to, &pool_info.lp_token, 0, &amount, &[]);

        // emit Withdraw(&sender, pool_id, amount, to);

        Ok(())
    }

    /// @notice Withdraw without caring about rewards. EMERGENCY ONLY.
    /// @param pool_id The index of the pool. See `poolInfo`.
    /// @param to Receiver of the LP tokens.
    #[endpoint]
    fn emergency_withdraw(&self, pool_id: u64, to: ManagedAddress) -> SCResult<()> {
        let sender = self.blockchain().get_caller();
        require!(!self.pool_info(pool_id).is_empty(), "Pool is not exist");
        let mut user_info = self.user_info(&sender).get();
        let amount = &user_info.amount.clone();
        require!(
            user_info.amount > BigUint::from(0u32),
            "emergency_withdraw: not good"
        );
        let pool_info = &mut self.pool_info(pool_id).get();

        //Effects
        user_info.amount = BigUint::from(0u32);
        user_info.reward_debt = BigUint::from(0u32);
        self.user_info(&sender).set(&user_info);

        // Interactions
        self.send()
            .direct(&to, &pool_info.lp_token, 0, &amount, &[]);

        // emit EmergencyWithdraw(&sender, pool_id, amount, to);

        Ok(())
    }

    // /// @notice Harvest proceeds for transaction sender to `to`.
    // /// @param pool_id The index of the pool. See `pool_info`.
    // /// @param to Receiver of rewards.
    #[endpoint]
    fn harvest(&self, pool_id: u64, to: ManagedAddress) -> SCResult<()> {
        let sender = self.blockchain().get_caller();
        require!(!self.pool_info(pool_id).is_empty(), "Pool is not exist");
        let pool_info = &mut self.pool_info(pool_id).get();
        self.update_pool(pool_info);
        let mut user_info = self.user_info(&sender).get();
        let accumulated_reward =
            &user_info.amount * &pool_info.acc_reward_per_share / ACC_REWARD_PRECISION;
        let pending_reward = &accumulated_reward - &user_info.reward_debt;

        // Effects
        user_info.reward_debt = accumulated_reward;
        self.user_info(&sender).set(&user_info);

        // Interactions
        self.send()
            .direct(&to, &self.reward().get(), 0, &pending_reward, &[]);

        // emit Harvest(&sender, pool_id, pending_reward, to);

        Ok(())
    }

    /* ========== RESTRICTED FUNCTIONS ========== */

    /// @notice Add a new LP to the pool. Can only be called by the owner.
    /// DO NOT add the same LP token more than once. Rewards will be messed up if you do.
    /// @param alloc_point AP of the new pool.
    /// @param lp_token Address of the LP token.
    #[only_owner]
    #[endpoint]
    fn add(&self, alloc_point: BigUint, lp_token: TokenIdentifier) -> SCResult<()> {
        let pool_length = self.pool_length().get();
        let block_timestamp = self.blockchain().get_block_timestamp();

        for i in 1..=pool_length {
            let pool_info = self.pool_info(i).get();
            require!(pool_info.lp_token != lp_token, "Cannot add existing pool");
        }

        self.total_alloc_point()
            .update(|total_alloc_point| *total_alloc_point += &alloc_point);

        let new_pool = PoolInfo {
            lp_token,
            acc_reward_per_share: BigUint::zero(),
            last_reward_time: block_timestamp,
            alloc_point,
        };

        self.pool_info(pool_length).set(&new_pool);

        // emit LogPoolAddition(pool_length, alloc_point, lp_token);

        Ok(())
    }

    /// @notice Update the given pool's reward allocation point. Can only be called by the owner.
    /// @param pool_id The index of the pool. See `pool_info`.
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

    /* ========== VIEW FUNCTION ========== */

    /// @notice View function to see pending reward on frontend.
    /// @param pool_id The index of the pool. See `pool_info`.
    /// @param user Address of user.
    /// @return pending reward for a given user.
    #[view(pendingReward)]
    fn pending_reward(&self, pool_id: u64, user: ManagedAddress) -> BigUint {
        let pool_info = self.pool_info(pool_id).get();
        let user_info = self.user_info(&user).get();
        let mut acc_reward_per_share = pool_info.acc_reward_per_share;
        let lp_supply = self.blockchain().get_sc_balance(&pool_info.lp_token, 0);
        let block_timestamp = self.blockchain().get_block_timestamp();
        if (block_timestamp > pool_info.last_reward_time && lp_supply > BigUint::from(0u32)) {
            let time: u64 = block_timestamp - &pool_info.last_reward_time;
            let reward_amount =
                BigUint::from(time) * self.reward_per_second().get() * &pool_info.alloc_point
                    / self.total_alloc_point().get();
            acc_reward_per_share = &reward_amount * ACC_REWARD_PRECISION / lp_supply;
        }
        let accumulated_reward = &user_info.amount * &acc_reward_per_share / ACC_REWARD_PRECISION;
        let pending_reward = &accumulated_reward - &user_info.reward_debt;
        pending_reward
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
