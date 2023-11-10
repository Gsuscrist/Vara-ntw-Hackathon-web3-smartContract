
#![no_std]
use gstd::{async_main, msg , prelude::*,ActorId};
use io::*;

#[cfg(feature = "binary-vendor")]
include!(concat!(env!("OUT_DIR"), "/wasm_binary.rs"));


#[derive(Debug, Clone, Default)]
struct Actors {  
    actors: HashMap<ActorId, u128>,
}

impl Actors {

    async fn destructor( &mut self, amount_tokens: u128){

        let currentstate = state_mut();

        let address_ft = addresft_state_mut();

        let payload = FTAction::Burn(amount_tokens);
     
        let result =  msg::send_for_reply_as::<_, FTEvent>(address_ft.ft_program_id,payload,0,0).expect("Error in sending a message").await;
        
        currentstate.entry(msg::source()).or_insert(amount_tokens); 

        let _ = match result {
            Ok(event) => match event {
                FTEvent::Ok => Ok(()),
                _ => Err(()),
            },
            Err(_) => Err(()),
        };
    }

    async fn creator(&mut self, amount_tokens: u128){

        let currentstate = state_mut();
        let address_ft = addresft_state_mut();           
        let payload = FTAction::Mint(amount_tokens);     
        let result =  msg::send_for_reply_as::<_, FTEvent>(address_ft.ft_program_id,payload,0,0).expect("Error in sending a message").await;
        currentstate.entry(msg::source()).or_insert(amount_tokens);  

        let _ = match result {
            Ok(event) => match event {
                FTEvent::Ok => Ok(()),
                _ => Err(()),
            },
            Err(_) => Err(()),
        };
    }

    async fn transfer(&mut self, amount_tokens: u128) {
 
        let currentstate = state_mut();
        let address_ft = addresft_state_mut();           
        let payload = FTAction::Transfer{from: exec::program_id(), to: msg::source() ,amount: amount_tokens};
        let _ = msg::send(address_ft.ft_program_id, payload, 0);
        currentstate.entry(msg::source()).or_insert(amount_tokens);  
       

    }

   
}

static mut ACTOR:Option<Actors> = None;

struct Ticket{
    ticket: (String,String,ImportanceLevel),
}


// 1. Create the main state as a static variable.
static mut STATE:Option<HashMap<ActorId, u128>,Ticket> = None;
static mut ADDRESSFT:Option<MainStruct> = None;




// 2. Create the mutability function for your state.
fn actors_state_mut() -> &'static mut Actors  {
    unsafe { ACTORS.get_or_insert(Default::default()) }
}

fn state_mut() -> &'static mut HashMap<ActorId,u128> {
    let state = unsafe { STATE.as_mut()};
    unsafe { state.unwrap_unchecked() }

}

fn addresft_state_mut() -> &'static mut MainStruct {
    let addressft = unsafe { ADDRESSFT.as_mut()};
    unsafe { addressft.unwrap_unchecked() }
}


// Create a public State
// #[derive(Default, Encode, Decode, TypeInfo)]
// pub struct CustomStruct {
//     pub firstfield: String,
//     pub secondfield: u128,
//     pub thirdfield: ActorId,
// }

// // Create a implementation on State
// impl CustomStruct {
//     #[allow(dead_code)]
//     async fn firstmethod(&mut self) {}
//     #[allow(dead_code)]
//     async fn secondmethod(&mut self) { }
//     #[allow(dead_code)]
//     async fn thirdmethod(&mut self) {}
// }


// 3. Create the init() function of your contract.

#[no_mangle]
extern "C" fn init () {

    let config: MainStruct = msg::load().expect("Unable to decode InitFT");

    let _actors = Actors {
        ..Default::default()
    };

    if config.ft_program_id.is_zero() {
        panic!("FT program address can't be 0");
    }

    let mainstruct = MainStruct {
        ft_program_id: config.ft_program_id,
        tickets:config.tickets
    };

    unsafe {
        ADDRESSFT = Some(mainstruct);
    }

   unsafe { STATE = Some(HashMap::new());Ticket(String,String,ImportanceLevel)}

}


// 4.Create the main() function of your contract.
#[async_main]
async fn main(){

    let action: Action = msg::load().expect("Could not load Action");

    let actors = unsafe { ACTORS.get_or_insert(Actors::default()) };

    match action {
        Action::FTCreate(amount) =>  {
         

                actors.creator(amount).await;
               
 
            },
        Action::FTDestroy(amount) => {

                
                actors.destructor(amount).await;
                     
            }

        Action::FTTransfer(amount) => {
     
                actors.transfer(amount).await;
                
             
            }

        Action::CreateTicket(ticket)=>{
                actors.creator(ticket).await;
            }

        Action::DestroyTicket(ticket)=>{
                actors.destructor(ticket).await;
            }
           
            };

}


    
        


// 5. Create the state() function of your contract.
 
#[no_mangle]
extern "C" fn state() {
 
    let state: <ContractMetadata as Metadata>::State =
        state_mut().iter().map(|(k, v),t| (*k, *v, *t)).collect();
     
    msg::reply(state, 0).expect("failed to encode or reply from `state()`");
}
