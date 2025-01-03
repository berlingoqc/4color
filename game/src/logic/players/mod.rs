pub mod key_loader;

use key_loader::get_key_loader;
use protocol::generated::card::{Player, Vec3};
use x25519_dalek::{PublicKey, StaticSecret};

use crate::utils::hash_to_u64;

pub struct MyPlayerConfiguration {
    pub profile_public_key: Option<[u8; 32]>,
    pub wallet_path: String,
    pub name: String,
    pub position: [f32; 3],
}

#[derive(Clone)]
pub struct OtherPlayer {
    pub name: String,

    pub hash: u64,

    pub profile_public_key: PublicKey,

    pub pub_key: PublicKey,

    pub position: [f32; 3],
}

pub struct MyPlayer {
    pub name: String,

    pub hash: u64,

    pub profile_public_key: PublicKey,

    pub pub_key: PublicKey,
    pub private: StaticSecret,

    pub position: [f32; 3],
}

impl Default for MyPlayerConfiguration {
    fn default() -> Self {
        MyPlayerConfiguration {
            wallet_path: String::new(),
            name: String::new(),
            profile_public_key: None,
            position: [0.0; 3],
        }
    }
}

impl MyPlayer {
    pub fn load(config: MyPlayerConfiguration) -> Self {
        let (pub_key, secret) = get_key_loader(&config).unwrap().load_key_pair().unwrap();

        let profile_public_key = config
            .profile_public_key
            .map(|d| PublicKey::from(d))
            .or_else(|| Some(pub_key.clone()))
            .unwrap();

        MyPlayer {
            private: secret,
            pub_key: pub_key,
            hash: hash_to_u64(profile_public_key.as_bytes().to_vec()),
            profile_public_key: profile_public_key,
            name: config.name,
            position: config.position,
        }
    }

    pub fn to_other_player(&self) -> OtherPlayer {
        OtherPlayer {
            name: self.name.clone(),
            hash: hash_to_u64(self.profile_public_key.as_bytes().to_vec()),
            profile_public_key: self.profile_public_key.clone(),
            pub_key: self.pub_key.clone(),
            position: self.position,
        }
    }

    pub fn to_protoc_player(&self) -> Player {
        Player {
            name: self.name.clone(),
            hash: self.hash,
            profile_pub_key: self.profile_public_key.to_bytes().to_vec(),
            pub_key: self.profile_public_key.to_bytes().to_vec(),
            position: Some(Vec3 {
                x: self.position[0] as u32,
                y: self.position[1] as u32,
                z: self.position[2] as u32,
            }),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    macro_rules! test_case {
        ($fname:expr) => {
            concat!(env!("CARGO_MANIFEST_DIR"), "/../tests/", $fname).to_string()
        };
    }

    #[test]
    fn load_alice() {
        let alice_config = MyPlayerConfiguration {
            wallet_path: test_case!("/wallets/alice.json"),
            position: [0.0; 3],
            profile_public_key: None,
            name: "alice".into(),
        };

        let _ = MyPlayer::load(alice_config);
    }
}
