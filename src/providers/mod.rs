pub mod aws;
use serde_traitobject::{Deserialize, Serialize};

pub trait Provider: Serialize + Deserialize {}
