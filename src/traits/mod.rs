use self::{auth::Auth, services::Services, session::Session};

pub mod auth;
pub mod services;
pub mod session;

pub trait YggoxideImpl: Auth + Session + Services {}
