use std::ops::Deref;

use crate::structs::common::Uuid;

impl Deref for Uuid {
    type Target = uuid::Uuid;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
