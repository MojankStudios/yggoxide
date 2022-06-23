use crate::structs::yggdrasil::{Information, RuntimeMode, ServerStatus};

static VERSION: &'static str = "5.2.0";

impl Default for ServerStatus {
    fn default() -> Self {
        Self::OK
    }
}

impl Default for RuntimeMode {
    fn default() -> Self {
        Self::Production
    }
}

impl Default for Information {
    fn default() -> Self {
        Self {
            status: Default::default(),
            runtime: Default::default(),
            application_author: "insertish".to_string(),
            application_description: "Mojank Authentication Server.".to_string(),
            application_name: "yggoxide".to_string(),
            application_owner: "deez".to_string(),
            specification_version: VERSION.to_string(),
            implementation_version: VERSION.to_string(),
        }
    }
}
