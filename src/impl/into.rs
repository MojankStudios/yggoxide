use crate::structs::{
    session::Profile,
    yggdrasil::{AuthenticationProfile, User},
};

impl From<Profile> for AuthenticationProfile {
    fn from(val: Profile) -> Self {
        Self {
            name: val.name,
            id: val.id,
        }
    }
}

impl From<Profile> for User {
    fn from(val: Profile) -> Self {
        Self {
            id: val.id,
            username: val.name,
            properties: vec![],
        }
    }
}
