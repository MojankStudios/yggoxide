/// # Player Property
#[derive(Serialize, Deserialize, JsonSchema)]
pub enum PlayerProperty {
    #[serde(rename = "textures")]
    Textures {
        /// base64 string
        value: String,

        /// base64 string; signed data using Yggdrasil's private key
        signature: Option<String>,
    },
}

/// # Player Profile
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Profile {
    /// Profile identifier
    pub id: String,

    /// Player's username
    pub name: String,

    /// Additional player properties
    pub properties: Vec<PlayerProperty>,
}
