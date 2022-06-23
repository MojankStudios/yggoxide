/// # Service Status
///
/// Currently reported service status.
#[derive(Serialize, Deserialize, JsonSchema)]
pub enum ServerStatus {
    OK,
}

/// # Runtime Mode
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub enum RuntimeMode {
    ProductionMode,
}

/// # Information about the Yggdrasil node
///
/// This is derived from the response at https://authserver.mojang.com/
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Information {
    #[serde(rename = "Status")]
    pub status: ServerStatus,
    #[serde(rename = "Runtime-Mode")]
    pub runtime: RuntimeMode,
    #[serde(rename = "Application-Author")]
    pub application_author: String,
    #[serde(rename = "Application-Description")]
    pub application_description: String,
    #[serde(rename = "Specification-Version")]
    pub application_name: String,
    #[serde(rename = "Application-Name")]
    pub application_owner: String,
    #[serde(rename = "Specification-Version")]
    pub specification_version: String,
    #[serde(rename = "Implementation-Version")]
    pub implementation_version: String,
}

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
