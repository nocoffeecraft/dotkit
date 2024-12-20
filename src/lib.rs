use serde::{Serialize, Deserialize};

#[derive(Clone, Debug)]
pub struct Contract {
    pub name: String,
    pub typ: CT,
}

// CT - Contract Type
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum CT {
    Counter,
    Token,
    NFT,
    Swap,
    Voting,
}

impl Contract {
    pub fn default() -> Self {
        Self {
            name: "counter".to_string(),
            typ: CT::Counter,
        }
    }

    pub fn name(&mut self, name: &str) -> Self {
        self.name = name.into();
        self.clone()
    }

    pub fn ct(&mut self, ct: CT) -> Self {
        self.typ = ct;
        self.clone()
    }
}
