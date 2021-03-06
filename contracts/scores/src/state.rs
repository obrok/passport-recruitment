use schemars::JsonSchema;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    #[serde(serialize_with = "to_array", deserialize_with = "from_array")]
    pub scores: HashMap<(Addr, String), i32>,
}

pub fn to_array<S>(map: &HashMap<(Addr, String), i32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.collect_seq(map.iter())
}

pub fn from_array<'de, D>(deserializer: D) -> Result<HashMap<(Addr, String), i32>, D::Error>
where
    D: Deserializer<'de>,
{
    let mut map = HashMap::new();
    for (key, score) in Vec::<((Addr, String), i32)>::deserialize(deserializer)? {
        map.insert(key, score);
    }
    Ok(map)
}

pub const STATE: Item<State> = Item::new("state");
