use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(bound = "T: Serialize + DeserializeOwned")]
pub struct Response<T: Serialize + DeserializeOwned> {
    #[serde(rename = "requestUUID")]
    pub id: Uuid,

    #[serde(flatten)]
    pub body: T,
}

pub mod any {
    use crate::launcher::types::response::{
        check_server, error, get_profile_by_uuid, get_public_key, restore_token, Response,
    };
    use serde::{Deserialize, Serialize};

    pub type Any = Response<Kind>;

    #[derive(Serialize, Deserialize, Debug)]
    #[serde(tag = "type")]
    pub enum Kind {
        #[serde(rename = "restore")]
        RestoreToken(restore_token::RestoreToken),

        #[serde(rename = "getPublicKey")]
        GetPublicKey(get_public_key::GetPublicKey),

        #[serde(rename = "checkServer")]
        CheckServer(check_server::CheckServer),

        #[serde(rename = "profileByUUID")]
        GetProfileByUuid(get_profile_by_uuid::GetProfileByUuid),

        #[serde(rename = "error")]
        Error(error::Error),
    }
}

pub mod error {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Error {
        #[serde(rename = "error")]
        pub kind: String,
    }
}

pub mod restore_token {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct RestoreToken {
        #[serde(rename = "invalidTokens")]
        pub invalid_tokens: Vec<String>,
    }
}

pub mod get_public_key {
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GetPublicKey {
        #[serde(rename = "rsaPublicKey")]
        pub rsa_public_key: String,

        #[serde(rename = "ecdsaPublicKey")]
        pub ecdsa_public_key: String,
    }
}

pub mod check_server {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct CheckServer {
        pub uuid: Uuid,

        #[serde(rename = "playerProfile")]
        pub profile: Profile,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Profile {
        pub uuid: Uuid,
        pub username: String,
    }
}

pub mod get_profile_by_uuid {
    use serde::{Deserialize, Serialize};
    use uuid::Uuid;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct GetProfileByUuid {
        #[serde(rename = "playerProfile")]
        pub player_profile: Profile,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Profile {
        pub uuid: Uuid,
        pub username: String,
        pub assets: Assets,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Assets {
        #[serde(rename = "SKIN", skip_serializing_if = "Option::is_none")]
        pub skin: Option<skin::Skin>,

        #[serde(rename = "CAPE", skip_serializing_if = "Option::is_none")]
        pub cape: Option<cape::Cape>,
    }

    pub mod skin {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Skin {
            pub url: String,
            pub digest: String,

            #[serde(skip_serializing_if = "Option::is_none")]
            pub metadata: Option<metadata::Metadata>,
        }

        pub mod metadata {
            use serde::{Deserialize, Serialize};

            #[derive(Serialize, Deserialize, Debug)]
            pub struct Metadata {
                pub model: Model,
            }

            #[derive(Serialize, Deserialize, Debug)]
            pub enum Model {
                #[serde(rename = "slim")]
                Slim,
            }
        }
    }

    pub mod cape {
        use serde::{Deserialize, Serialize};

        #[derive(Serialize, Deserialize, Debug)]
        pub struct Cape {
            pub url: String,
        }
    }
}
