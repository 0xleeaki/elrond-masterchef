elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct PoolInfo<M: ManagedTypeApi> {
    pub lp_token: TokenIdentifier<M>,
    pub acc_reward_per_share: BigUint<M>,
    pub last_reward_time: u64,
    pub alloc_point: BigUint<M>,
}
