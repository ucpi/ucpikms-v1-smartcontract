use cosmwasm_std::{to_binary, Api, Binary, Env, Extern, HandleResponse, InitResponse, Querier, StdError, StdResult, Storage};
use crate::msg::{HandleMsg, InitMsg, QueryMsg, HandleAnswer, QueryAnswer};
use crate::state::{load, may_load, save,Contractdetail, CONFIG_KEY, Mainkey, nodevote, token, viewkey};

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
        HandleMsg::Addkey { key, token } => {addkey(deps, env,key,token)}
        HandleMsg::Votefor { token, vote } => {
            voting(deps,env,token,vote)
        },
        HandleMsg::login { jw, vk }=>{
            logn(deps,env,jw,vk)
        }
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
        QueryMsg::Nodevot { jwt,nodeadress }=>{
            let mut sen:String=jwt;
            let bs: &str = &&&nodeadress;
            sen.push_str(bs);
            let keyy:&[u8]=sen.as_bytes();
            let config:nodevote  = load(&deps.storage, keyy)?;
            to_binary(&QueryAnswer::Nodevte { agree: config.voteres })
            
        },
        QueryMsg::Getvoting { jwt}=>{
            
            let config:token  = load(&deps.storage, jwt.as_bytes())?;
            to_binary(&QueryAnswer::Voting { numyvote: config.numyvotes,numnovote:config.numnovotes })
        },
     QueryMsg::Showkey { vik }=>{
        let mut mg="".to_string();
        let mut ky="".to_string();
        let result:Option<viewkey>= may_load(&deps.storage,vik.as_bytes()).ok().unwrap();
        match result{
            Some(a)=>{
                let con2:token=load(&deps.storage, a.token.as_bytes())?;
                if(con2.numyvotes>1){
                  
                    let keyy:&[u8]=b"0xpranjl";
                    let confg: viewkey = load(&deps.storage, keyy)?;
                    let con3:token=load(&deps.storage, confg.token.as_bytes())?;
                     
                    if(con3.numyvotes>1){
                    mg="Successfully fetched key!".to_string();
                    ky=confg.key;
                    }
                    else{
                        mg="key not verified".to_string();
                        ky="key not verified".to_string();
                    }
                }
               else{
                mg=String::from("invalid access not enough votes");
                ky=String::from("NA");
               }   
            }
         None=>{
         mg=String::from("key not found");
         ky=String::from("NA");
         }
        }
        to_binary(&QueryAnswer::Kyy { msg: mg, key:ky })  
     }
        // add query execution code here
        
    }
  
        // add query execution code here
    
}

fn addkey<S: Storage, A: Api, Q: Querier>(
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
    let tkn=token.clone();
    let tknn=tkn.as_bytes();
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
    let vto=token{
        jwt:tkn.clone(),
        numyvotes:0,
        numnovotes:0,
    };
    save(&mut deps.storage, tknn,&vto)?;
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
fn voting<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    token:String,
    vote:bool,
) -> StdResult<HandleResponse> {
    let mut status:String="".to_string();
    let mut mssg:String=String::from("").to_string();
    let mut verifie:bool;
    let tkn=token.clone();
    let mut tokn=token.clone();
    let node1:String=String::from("secret1er4t8huuwpdn3pqc4wsk3dmgg0lq0c7hryrkej");
    let node2:String=String::from("secret1kqwzrue6hax8rqnappaaad4a8ug42v4nr990ae");
    let node3:String=String::from("secret13ln0gaqcqrykvj3znazd877zm3lgjlh85zzeca");
    let mut bs:&str=&env.message.sender.to_string();
    let mut sen: String = token.to_string();
    sen.push_str(bs);
    let keyy:&[u8]=sen.as_bytes();
    
    // let sender_address = deps.api.canonical_address(&env.message.sender)?;
    if env.message.sender.to_string()==node1||env.message.sender.to_string()==node2||env.message.sender.to_string()==node3{
        let mut config: token = load(&mut deps.storage, token.as_bytes())?;
        let no=config.numnovotes;
        let yes=config.numyvotes;
        let noi=config.numnovotes+1;
        let yesi:i32=config.numyvotes+1;
        if(config.numnovotes+config.numyvotes<3){
        let result:Option<nodevote>= may_load(&mut deps.storage,&keyy).ok().unwrap();
    match result{
        Some(x)=>{
      mssg="you already voted".to_string();
    }
    None=>{
        let nv=nodevote{
            voteres: vote,
        };
        save(&mut deps.storage, keyy, &nv)?;
    if vote==false {
        let vto=token{
            jwt:token,
            numyvotes:yes,
            numnovotes:noi,
        };
        save(&mut deps.storage, tkn.as_bytes(), &vto)?;
    }
    else{
        let vto=token{
            jwt:token,
            numyvotes:yesi,
            numnovotes:no,
        };
        save(&mut deps.storage, tkn.as_bytes(), &vto)?;
    }
    }

    }
}
else{
  mssg="voting is over".to_string();
}
    }
else{
    mssg="you are not authorised to vote".to_string();
}
Ok(HandleResponse {
    messages: vec![],
    log: vec![],
    data: Some(to_binary(&HandleAnswer::Vote{
        msg:mssg,
    })?),
})
}

fn logn<S: Storage, A: Api, Q: Querier>(
    deps: &mut Extern<S, A, Q>,
    env: Env,
    jw:String,
    vw:String
) -> StdResult<HandleResponse> {
    let result:Option<Mainkey>= may_load(&mut deps.storage,jw.as_bytes()).ok().unwrap();
    let mut mg="".to_string();
    let mut jv=jw;
    let mut vv=vw;
    match result {
    Some(a)=>{
        mg="JWT previously used".to_string();
    }
    None=>{
        mg="login request and viewing key added".to_string();
        let vto=token{
            jwt:jv.clone(),
            numyvotes:0,
            numnovotes:0,
        };
        save(&mut deps.storage, jv.as_bytes(), &vto)?;
        let vko=viewkey{
            key:vv.clone(),
            token:jv.clone(),
        };
        save(&mut deps.storage, vv.as_bytes(), &vko)?;
    }
    }
    Ok(HandleResponse {
        messages: vec![],
        log: vec![],
        data: Some(to_binary(&HandleAnswer::addlogin{
            msg:mg,
        })?),
    })
}
