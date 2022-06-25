use iso8601_timestamp::Timestamp;

/// Bearer <access_token>
pub struct AccessToken(pub String);

/// Boolean toggle struct
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Toggle {
    enabled: bool,
}

/// Player Privileges
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPrivileges {
    /// Wether the player can chat
    online_chat: Toggle,

    /// Wether the player can join multiplayer servers
    multiplayer_server: Toggle,

    /// Wether the player can join realms
    multiplayer_realms: Toggle,

    /// Wether telemetry is on
    telemetry: Toggle,
}

/// Profanity filter preferences
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ProfanityFilterPreferences {
    profanity_filter_on: bool,
}

/// Ban reason
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum BanReason {
    HateSpeech,
    TerrorismOrViolentExtremism,
    ChildSexualExploitationOrAbuse,
    ImminentHarm,
    NonConsensualIntimateImagery,
    HarassmentOrBullying,
    DefamationImpersonationFalseInformation,
    SelfHarmOrSuicide,
    AlcoholTobaccoDrugs,
}

/// Ban information
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Ban {
    ban_id: String,
    expires: usize,
    reason: BanReason,
    reason_message: String,
}

/// Ban scopes
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "UPPERCASE")]
pub struct BanScopes {
    multiplayer: Ban,
}

/// User ban status
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct BanStatus {
    banned_scopes: BanScopes,
}

/// Player attributes
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAttributes {
    privileges: PlayerPrivileges,
    profanity_filter_preferences: ProfanityFilterPreferences,
    ban_status: BanStatus,
}

/// Key pair
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Keypair {
    private_key: String,
    public_key: String,
}

/// Key pair
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCertificate {
    key_pair: Keypair,

    /// Signed base64 string
    public_key_signature: String,

    /// Time at which this certificate expires
    expires_at: Timestamp,

    /// Time at which this certificate is refreshed
    refreshed_after: Timestamp,
}
