elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, TypeAbi)]
pub struct UserInfo<M: ManagedTypeApi> {
    pub amount: BigUint<M>,
    pub reward_debt: BigUint<M>,
}
