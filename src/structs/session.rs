/// # Minecraft Skin Model Variant
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub enum ModelVariant {
    /// Classic "Steve" skin model
    Classic,
    /// New "Alex" skin model
    Slim,
}

/// # Texture Metadata
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct TextureMetadata {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<ModelVariant>,
}

/// # Texture Information
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct TextureInformation {
    /// Direct URL to texture
    pub url: String,

    /// Optional metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<TextureMetadata>,
}

/// # Player Textures
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "UPPERCASE")]
pub struct PlayerTextureDefinitions {
    pub skin: Option<TextureInformation>,
    pub cape: Option<TextureInformation>,
}

/// # Player Textures
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerTextures {
    /// Timestamp at which these were changed
    pub timestamp: usize,

    /// Player's UUID
    pub profile_id: String,

    /// Player's display name
    pub profile_name: String,

    /// Texture definitions
    pub textures: PlayerTextureDefinitions,
}

/// # Player Property
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
#[serde(tag = "name")]
pub enum PlayerProperty {
    #[serde(rename = "textures")]
    Textures {
        /// base64 string
        value: String,

        /// base64 string; signed data using Yggdrasil's private key
        #[serde(skip_serializing_if = "Option::is_none")]
        signature: Option<String>,
    },
}

/// # Player Profile
#[derive(Serialize, Deserialize, JsonSchema, Clone)]
pub struct Profile {
    /// Profile identifier
    pub id: String,

    /// Player's username
    pub name: String,

    /// Additional player properties
    pub properties: Vec<PlayerProperty>,
}
