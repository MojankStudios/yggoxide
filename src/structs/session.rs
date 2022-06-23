/// # Player Property
#[derive(Serialize, Deserialize, JsonSchema)]
pub enum PlayerProperty {
    #[serde(rename = "textures")]
    Textures {
        /// base64 string
        value: String,

        /// base64 string; signed data using Yggdrasil's private key
        signature: String,
    },
}

/// # Player Session
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Session {
    /// Profile identifier
    id: String,

    /// Player's username
    name: String,

    /// Additional player properties
    properties: Vec<PlayerProperty>,
}
