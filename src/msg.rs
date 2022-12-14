use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InitMsg {

    pub owner: String,
    pub idtype: String

    // add InitMsg parameters here
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleMsg {
    // add HandleMsg types here
    Votefor{    
        token:String,
        vote:bool,
    },
    Addkey{
    key:String,
        token:String,
    },
    login{
        jw:String,
        vk:String,
    }



}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // add QueryMsg types here
    Getcontractdetail {},
    Getvoting{
        jwt:String,

    },
    Nodevot{
        jwt:String,
        nodeadress:String, 
     },
     Showkey{
      vik:String,
     },
 
}

/// Responses from handle function
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum HandleAnswer {
// add HandleMsg response types here
Addkey{
    status:bool,
    msg:String,
    verified:bool,
},
Vote{
    msg:String,
},
addlogin{
    msg:String,
}
}

/// Responses from query function
#[derive(Serialize, Deserialize, Debug, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryAnswer {
    Contractdetail {
        owner: String,
         idtype: String,
    },
    Voting{
        numyvote:i32,
        numnovote:i32,
    },
    Nodevte{
        agree:bool,
    },
    Kyy{
        msg:String,
        key:String,
    }
    // add QueryMsg response types here
}
