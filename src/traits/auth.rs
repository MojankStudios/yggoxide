use crate::{
    api::auth::{
        authenticate::{PayloadAuthenticate, ResponseAuthenticate},
        invalidate::PayloadInvalidate,
        refresh::{PayloadRefresh, ResponseRefresh},
        signout::PayloadSignout,
        validate::PayloadValidate,
    },
    Result,
};

#[async_trait]
pub trait Auth: Sync + Send {
    /// Authenticate a user with the service
    async fn authenticate(&self, data: PayloadAuthenticate) -> Result<ResponseAuthenticate>;

    /// Refresh an access token
    async fn refresh(&self, data: PayloadRefresh) -> Result<ResponseRefresh>;

    /// Validate an access token
    async fn validate(&self, data: PayloadValidate) -> Result<()>;

    /// Sign out using credentials
    async fn signout(&self, data: PayloadSignout) -> Result<()>;

    /// Invalidate current access token
    async fn invalidate(&self, data: PayloadInvalidate) -> Result<()>;
}
