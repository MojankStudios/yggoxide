use std::ops::Add;

use iso8601_timestamp::Timestamp;

use crate::structs::{
    common::{Information, RuntimeMode, ServerStatus, Uuid},
    services::{Keypair, PlayerCertificate, PlayerPrivileges, Toggle},
};

static VERSION: &str = "5.2.0";

impl Default for Uuid {
    fn default() -> Self {
        Self(uuid::Uuid::new_v4())
    }
}

impl Default for ServerStatus {
    fn default() -> Self {
        Self::OK
    }
}

impl Default for RuntimeMode {
    fn default() -> Self {
        Self::ProductionMode
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

impl Toggle {
    pub fn y() -> Self {
        Self { enabled: true }
    }

    pub fn n() -> Self {
        Self { enabled: false }
    }
}

impl Default for PlayerPrivileges {
    fn default() -> Self {
        Self {
            online_chat: Toggle::y(),
            multiplayer_server: Toggle::y(),
            multiplayer_realms: Toggle::y(),
            telemetry: Toggle::n(),
        }
    }
}

impl Default for Keypair {
    fn default() -> Self {
        use rsa::{
            pkcs8::{EncodePrivateKey, EncodePublicKey, LineEnding},
            RsaPrivateKey,
        };

        let mut rng = rand::thread_rng();
        let priv_key = RsaPrivateKey::new(&mut rng, 2048).expect("failed to generate key");
        let priv_der = EncodePrivateKey::to_pkcs8_der(&priv_key)
            .expect("failed to convert private key to der format");
        let pub_der = EncodePublicKey::to_public_key_der(&*priv_key)
            .expect("failed to convert public key to der format");

        let private_key =
            pem_rfc7468::encode_string("RSA PRIVATE KEY", LineEnding::LF, priv_der.as_ref())
                .expect("failed to convert private key to pem format");
        let public_key =
            pem_rfc7468::encode_string("RSA PUBLIC KEY", LineEnding::LF, pub_der.as_ref())
                .expect("failed to convert public key to pem format");

        Self {
            private_key,
            public_key,
        }
    }
}

impl Default for PlayerCertificate {
    fn default() -> Self {
        use std::time::{Duration, SystemTime, UNIX_EPOCH};

        Self {
            key_pair: Default::default(),
            public_key_signature: Default::default(),
            expires_at: Timestamp::from_unix_timestamp_ms(
                SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .add(Duration::from_secs(86400))
                    .as_millis()
                    .try_into()
                    .expect("Time too big"),
            ),
            refreshed_after: Timestamp::now_utc(),
        }
    }
}
