/// # Yggdrasil client information
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Agent {
    /// Name of the remote client
    pub name: String,

    /// Protocol version of the client
    pub version: usize,
}

/// # User property
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(tag = "name")]
pub enum UserProperty {
    /// Preferred language used by this user
    #[serde(rename = "preferredLanguage")]
    PreferredLanguage {
        /// Valid language code
        value: String,
    },
    /// Country of account registration
    #[serde(rename = "registrationCountry")]
    RegistrationCountry {
        /// 2L country code
        value: String,
    },
    /// Twitch token credentials if associated
    #[serde(rename = "twitch_access_token")]
    TwitchAccessToken {
        /// Valid OAuth 2.0 token
        value: String,
    },
}

/// # User
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct User {
    /// Username
    pub username: String,

    /// Additional user properties
    pub properties: Vec<UserProperty>,

    /// The `remoteID` for the user
    pub id: String,
}

/// # Authentication Profile
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AuthenticationProfile {
    /// Username
    pub name: String,

    /// UUID of the account
    pub id: String,
}
