#[metawasm]
pub mod metafns {
    pub type State = <ContractMetadata as Metadata>::State;
    pub fn get_state(state: State) -> Vec<(ActorId, u128),ticket> {
        state
    }


}
