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
