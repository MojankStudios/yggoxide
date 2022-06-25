#[derive(Debug, Serialize, Deserialize, JsonSchema, Default)]
pub struct InnerError {
    #[serde(rename = "errorMessage")]
    error_message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cause: Option<String>,
}

/// Yggdrasil error variants
///
/// Refer to https://wiki.vg/Authentication#Errors on how to construct these.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "error")]
pub enum Error {
    #[serde(rename = "Method Not Allowed")]
    MethodNotAllowed(InnerError),
    #[serde(rename = "Not Found")]
    NotFound(InnerError),
    ForbiddenOperationException(InnerError),
    IllegalArgumentException(InnerError),
    #[serde(rename = "Unsupported Media Type")]
    UnsupportedMediaType(InnerError),
    ResourceException(InnerError),
    GoneException(InnerError),
}

pub type Result<T> = std::result::Result<T, Error>;
