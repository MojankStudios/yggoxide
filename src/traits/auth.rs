use crate::{
    api::auth::authenticate::{PayloadAuthenticate, ResponseAuthenticate},
    Result,
};

#[async_trait]
pub trait Auth {
    async fn authenticate(data: PayloadAuthenticate) -> Result<ResponseAuthenticate>;
}
