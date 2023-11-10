
#![no_std]
use gstd::{ prelude::*, ActorId };
use gmeta::{In,InOut,Metadata};


#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum ImportanceLevel{
    Low = 1,
    Mid = 2,
    High =3,
}

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Ticket{
    pub name: String,
    pub description : String,
    pub importance_level:  ImportanceLevel,

}



pub struct ContractMetadata;

// 5. Define the structure of actions, events and state for your metadata.
impl Metadata for ContractMetadata{
     type Init = In<MainStruct>;
     type Handle = InOut<Action,Event>;
     type Others = ();
     type Reply=();
     type Signal = ();
     type State = Vec<(ActorId, u128)>;

}


// 3. Create your own Struct
pub type TransactionId = u64;

#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub struct Transaction<T> {
    pub id: TransactionId,
    pub action: T,
}



// 4. Create your init Struct
#[derive(Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub struct MainStruct {
   
    // Example:
    pub ft_program_id: ActorId,

    pub tickets : Ticket,


}



// 1. Create your own Actions
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
pub enum Action {
    
    CreateTicket(Ticket),
    DestroyTicket(Ticket),
    FTCreate(u128),
    FTDestroy(u128),
    FTTransfer(u128),
    
    
}


// 2. Create your own Events
#[derive(Encode, Decode, TypeInfo, Hash, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum  Event {

    SuccessfulTicketCreation,
    SuccessfulTicketDestruction,
    SuccessfulFTCreation,
    SuccessfulFTDestruction,
    SuccessfulFTTransaccion,

}



// Create child Actions
    //ft
#[derive(Debug, Decode, Encode, TypeInfo)]
#[codec(crate = gstd::codec)]
#[scale_info(crate = gstd::scale_info)]
pub enum FTAction {
    Mint(u128),
    Burn(u128),
    Transfer {
        from: ActorId,
        to: ActorId,
        amount: u128,
    },
    Approve {
        to: ActorId,
        amount: u128,
    },
    TotalSupply,
    BalanceOf(ActorId),
}


// Create child events
    //ft
#[derive(Encode, Decode, TypeInfo)]
pub enum FTEvent {
    Ok,
    Err,
    Balance(u128),
    PermitId(u128),
}


//extra
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
pub enum Error {
    ZeroAmount,
    ZeroReward,
    ZeroTime,
    TransferTokens,
    PreviousTxMustBeCompleted,
    InsufficentBalance,
    NotOwner,
    StakerNotFound,
    ContractError(String),
}