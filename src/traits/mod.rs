use self::{auth::Auth, session::Session};

pub mod auth;
pub mod session;

pub trait YggoxideImpl: Auth + Session {}
