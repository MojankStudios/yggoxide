use iso8601_timestamp::Timestamp;

use super::common::Uuid;

/// Bearer <access_token>
pub struct AccessToken(pub String);

/// Boolean toggle struct
#[derive(Serialize, Deserialize, JsonSchema)]
pub struct Toggle {
    pub enabled: bool,
}

/// Player Privileges
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlayerPrivileges {
    /// Wether the player can chat
    pub online_chat: Toggle,

    /// Wether the player can join multiplayer servers
    pub multiplayer_server: Toggle,

    /// Wether the player can join realms
    pub multiplayer_realms: Toggle,

    /// Wether telemetry is on
    pub telemetry: Toggle,
}

/// Profanity filter preferences
#[derive(Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProfanityFilterPreferences {
    pub profanity_filter_on: bool,
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
    pub ban_id: Uuid,
    pub expires: usize,
    pub reason: BanReason,
    pub reason_message: String,
}

/// Ban scopes
#[derive(Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "UPPERCASE")]
pub struct BanScopes {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiplayer: Option<Ban>,
}

/// User ban status
#[derive(Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct BanStatus {
    pub banned_scopes: BanScopes,
}

/// Player attributes
#[derive(Serialize, Deserialize, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct PlayerAttributes {
    pub privileges: PlayerPrivileges,
    pub profanity_filter_preferences: ProfanityFilterPreferences,
    pub ban_status: BanStatus,
}

/// Key pair
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Keypair {
    pub private_key: String,
    pub public_key: String,
}

/// Player certificate
#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct PlayerCertificate {
    /// The keypair
    pub key_pair: Keypair,

    /// Signed base64 string
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public_key_signature: Option<String>,

    /// Time at which this certificate expires
    pub expires_at: Timestamp,

    /// Time at which this certificate was refreshed
    pub refreshed_after: Timestamp,
}
