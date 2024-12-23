use serde::{Deserialize, Serialize};

/// Contract Details
#[derive(Clone, Debug)]
pub struct Contract {
    /// Name of the contract/project (default: counter)
    pub name: String,

    /// Auther Name
    pub a_name: String,

    /// Author email address
    pub a_email: String,

    /// Contract Type (default: Counter)
    pub typ: CT,
}

// CT - Contract Type
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum CT {
    /// Basic Counter Contract
    Counter,

    /// ERC20 Token Contract
    Token,

    /// Non-Fungible Token Contract
    NFT,

    /// Basic Multisig Contract
    Multisig,
}

/// Follows Builder Pattern
impl Contract {
    pub fn default() -> Self {
        Self {
            name: "counter".to_string(),
            typ: CT::Counter,
            a_name: "[your_name]".to_string(),
            a_email: "[your_email]".to_string(),
        }
    }

    /// Update contract/project name
    pub fn name(mut self, name: &str) -> Self {
        self.name = name.into();
        self
    }

    /// Update contract type
    pub fn ct(mut self, ct: CT) -> Self {
        self.typ = ct;
        self
    }

    /// Update author name
    pub fn a_name(mut self, a_name: &str) -> Self {
        self.a_name = a_name.into();
        self
    }

    /// Update author email address
    pub fn a_email(mut self, a_email: &str) -> Self {
        self.a_email = a_email.into();
        self
    }
}
