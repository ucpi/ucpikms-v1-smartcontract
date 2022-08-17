use cosmwasm_std::{to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage};
use std::convert::TryFrom;
use crate::msg::{HandleMsg, InitMsg, QueryMsg, HandleAnswer, QueryAnswer};
use crate::state::{load, may_load, save, State, Reminder,Contractdetail, CONFIG_KEY, Mainkey};

pub fn init<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state=Contractdetail{
    owner:msg.owner,
        idtype:msg.idtype,
    };
    save(&mut deps.storage, CONFIG_KEY, &state)?;
    Ok(InitResponse::default())

    // add init constructor functionality here
}

pub fn handle<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    msg: HandleMsg,
) -> StdResult<HandleResponse> {
 
   match msg {
        HandleMsg::Addkey { key, token } => {try_read(deps, env,key,token)}
        HandleMsg::Votefor { owner, token, vote } => todo!(),
    }
}

pub fn query<S: Storage, A: Api, Q: Querier>(
    deps: &Extern<S, A, Q>,
    msg: QueryMsg,
) -> StdResult<Binary> {

    match msg {
        QueryMsg::Getcontractdetail {}=>{
            let config:  Contractdetail  = load(&deps.storage, CONFIG_KEY)?;
            to_binary(&QueryAnswer::Contractdetail{ owner: config.owner,idtype:config.idtype})  
        }
        // add query execution code here
    }
}

fn try_read<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    key:String,
    token:String,
) -> StdResult<HandleResponse> {
    // let mut status:Option<bool>=None;
    // let mut mssg:Option<String>=None;
    // let mut verifie:Option<bool>=None;
    let mut status:bool;
    let mut mssg:String=String::from("").to_string();
    let mut verifie:bool;
    let keyy:&[u8]=b"0xpranjl";
   // let sender_address = deps.api.canonical_address(&env.message.sender)?;
    let result:Option<Mainkey>= may_load(&mut deps.storage,&keyy).ok().unwrap();
match result {
    Some(keyp)=>{
        if keyp.verified {
            // mssg=Some("Key already present!!".to_string());
            // status=Some(false);
            // verifie=Some(true); 
            mssg="Key already present!!".to_string();
            status=false;
            verifie=true; 
        }
        else{
            // mssg=Some("Key already present waiting for confirmation!!".to_string());
            // status=Some(false);
            // verifie=Some(false); 
            mssg="Key already present waiting for confirmation!!".to_string();
            status=false;
            verifie=false; 
        }
    
  
    }
None=>{
    let data=Mainkey{
        key:key,
        token:token,
        verified:false,
    };
    save(&mut deps.storage, keyy, &data)?;
    // mssg = Some(String::from("Key added and waiting for confirmation!!"));
    // status=Some(true);
    // verifie=Some(false); 
    mssg = String::from("Key added and waiting for confirmation!!");
    status=true;
    verifie=false; 
}  
}
Ok(HandleResponse {
    messages: vec![],
    log: vec![],
    data: Some(to_binary(&HandleAnswer::Addkey{
        status:status,
        msg:mssg,
        verified:verifie,
    })?),
})
}
