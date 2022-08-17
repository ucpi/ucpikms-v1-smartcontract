use cosmwasm_std::{to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage};
use std::convert::TryFrom;
use crate::msg::{HandleMsg, InitMsg, QueryMsg, HandleAnswer, QueryAnswer};
use crate::state::{load, may_load, save, State, Reminder,Contractdetail, CONFIG_KEY};

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
    todo!();
    // match msg {
    //
    //     // add handle transaction execution code here
    // }
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
