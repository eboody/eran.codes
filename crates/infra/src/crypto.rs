use std::collections::BTreeMap;

use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use domain::sensitive;
use rand_core::{OsRng, RngCore};
use snafu::prelude::*;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SealedValue {
    pub key_id: sensitive::KeyId,
    pub nonce: Vec<u8>,
    pub ciphertext: Vec<u8>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyMaterial {
    pub key_id: sensitive::KeyId,
    pub key: [u8; 32],
}

#[derive(Clone)]
pub struct Keyring {
    active_key_id: sensitive::KeyId,
    entries: BTreeMap<sensitive::KeyId, KeyEntry>,
}

#[derive(Clone)]
struct KeyEntry {
    status: sensitive::CipherKeyStatus,
    cipher: Aes256Gcm,
}

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("duplicate encryption key id in configured keyring: {key_id}"))]
    DuplicateKeyId { key_id: sensitive::KeyId },
    #[snafu(display("configured active encryption key id is missing: {key_id}"))]
    UnknownActiveKeyId { key_id: sensitive::KeyId },
    #[snafu(display("configured active encryption key id is disabled: {key_id}"))]
    DisabledActiveKeyId { key_id: sensitive::KeyId },
    #[snafu(display("configured disabled encryption key id is missing: {key_id}"))]
    UnknownDisabledKeyId { key_id: sensitive::KeyId },
    #[snafu(display("stored ciphertext references unknown encryption key id: {key_id}"))]
    UnknownKeyId { key_id: sensitive::KeyId },
    #[snafu(display("stored ciphertext references disabled encryption key id: {key_id}"))]
    DisabledKeyId { key_id: sensitive::KeyId },
    #[snafu(display("invalid AES-GCM nonce length: expected 12 bytes, got {len}"))]
    InvalidNonceLength { len: usize },
    #[snafu(display("AES-GCM encryption failed"))]
    Encrypt,
    #[snafu(display("AES-GCM decryption failed for key id {key_id}"))]
    Decrypt { key_id: sensitive::KeyId },
}

impl Keyring {
    pub fn new(
        key_materials: Vec<KeyMaterial>,
        active_key_id: sensitive::KeyId,
        disabled_key_ids: &[sensitive::KeyId],
    ) -> Result<Self> {
        let mut entries = BTreeMap::new();
        for key_material in key_materials {
            if entries.contains_key(&key_material.key_id) {
                return Err(Error::DuplicateKeyId {
                    key_id: key_material.key_id,
                });
            }

            let status = if disabled_key_ids.contains(&key_material.key_id) {
                sensitive::CipherKeyStatus::Disabled
            } else if key_material.key_id == active_key_id {
                sensitive::CipherKeyStatus::Active
            } else {
                sensitive::CipherKeyStatus::ReadOnlyLegacy
            };

            entries.insert(
                key_material.key_id,
                KeyEntry {
                    status,
                    cipher: Aes256Gcm::new((&key_material.key).into()),
                },
            );
        }

        let Some(active_entry) = entries.get(&active_key_id) else {
            return Err(Error::UnknownActiveKeyId { key_id: active_key_id });
        };
        if active_entry.status == sensitive::CipherKeyStatus::Disabled {
            return Err(Error::DisabledActiveKeyId { key_id: active_key_id });
        }
        for disabled_key_id in disabled_key_ids {
            if !entries.contains_key(disabled_key_id) {
                return Err(Error::UnknownDisabledKeyId {
                    key_id: disabled_key_id.clone(),
                });
            }
        }

        Ok(Self {
            active_key_id,
            entries,
        })
    }

    pub fn active_key_id(&self) -> &sensitive::KeyId {
        &self.active_key_id
    }

    pub fn configured_keys(&self) -> Vec<sensitive::ConfiguredKey> {
        self.entries
            .iter()
            .map(|(key_id, entry)| {
                sensitive::ConfiguredKey::builder()
                    .key_id(key_id.clone())
                    .status(entry.status)
                    .build()
            })
            .collect()
    }

    pub fn encrypt(&self, plaintext: impl AsRef<[u8]>) -> Result<SealedValue> {
        let mut nonce_bytes = [0_u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let entry = self
            .entries
            .get(&self.active_key_id)
            .expect("active key must exist after keyring initialization");
        let ciphertext = entry
            .cipher
            .encrypt(Nonce::from_slice(&nonce_bytes), plaintext.as_ref())
            .map_err(|_| Error::Encrypt)?;

        Ok(SealedValue {
            key_id: self.active_key_id.clone(),
            nonce: nonce_bytes.to_vec(),
            ciphertext,
        })
    }

    pub fn decrypt(&self, sealed: &SealedValue) -> Result<Vec<u8>> {
        let Some(entry) = self.entries.get(&sealed.key_id) else {
            return Err(Error::UnknownKeyId {
                key_id: sealed.key_id.clone(),
            });
        };
        if entry.status == sensitive::CipherKeyStatus::Disabled {
            return Err(Error::DisabledKeyId {
                key_id: sealed.key_id.clone(),
            });
        }

        let nonce =
            Nonce::from_slice(sealed.nonce.get(..12).ok_or(Error::InvalidNonceLength {
                len: sealed.nonce.len(),
            })?);
        entry
            .cipher
            .decrypt(nonce, sealed.ciphertext.as_ref())
            .map_err(|_| Error::Decrypt {
                key_id: sealed.key_id.clone(),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, KeyMaterial, Keyring, SealedValue};
    use domain::sensitive;

    fn key_id(value: &str) -> sensitive::KeyId {
        sensitive::KeyId::try_new(value).expect("key id")
    }

    fn keyring(
        disabled: &[sensitive::KeyId],
    ) -> Keyring {
        Keyring::new(
            vec![
                KeyMaterial {
                    key_id: key_id("legacy_data_key"),
                    key: [7_u8; 32],
                },
                KeyMaterial {
                    key_id: key_id("active_data_key"),
                    key: [8_u8; 32],
                },
            ],
            key_id("active_data_key"),
            disabled,
        )
        .expect("keyring")
    }

    #[test]
    fn round_trip_succeeds_with_active_key() {
        let keyring = keyring(&[]);
        let sealed = keyring.encrypt("hello").expect("encrypted");
        let decrypted = keyring.decrypt(&sealed).expect("decrypted");

        assert_eq!(sealed.key_id.to_string(), "active_data_key");
        assert_eq!(decrypted, b"hello");
    }

    #[test]
    fn readable_legacy_key_can_decrypt_old_ciphertext() {
        let legacy_keyring = Keyring::new(
            vec![KeyMaterial {
                key_id: key_id("legacy_data_key"),
                key: [7_u8; 32],
            }],
            key_id("legacy_data_key"),
            &[],
        )
        .expect("legacy keyring");
        let current_keyring = keyring(&[]);
        let legacy_ciphertext = legacy_keyring.encrypt("hello").expect("encrypted");
        let decrypted = current_keyring.decrypt(&legacy_ciphertext).expect("decrypted");

        assert_eq!(decrypted, b"hello");
    }

    #[test]
    fn unknown_key_id_fails_closed() {
        let keyring = keyring(&[]);
        let error = keyring
            .decrypt(&SealedValue {
                key_id: key_id("missing_key"),
                nonce: vec![0_u8; 12],
                ciphertext: vec![1, 2, 3],
            })
            .expect_err("decrypt should fail");

        assert!(matches!(error, Error::UnknownKeyId { .. }));
    }

    #[test]
    fn disabled_key_id_fails_closed() {
        let disabled_key_id = key_id("legacy_data_key");
        let legacy_ciphertext = Keyring::new(
            vec![KeyMaterial {
                key_id: disabled_key_id.clone(),
                key: [7_u8; 32],
            }],
            disabled_key_id.clone(),
            &[],
        )
        .expect("legacy keyring")
        .encrypt("hello")
        .expect("encrypted");
        let keyring = keyring(std::slice::from_ref(&disabled_key_id));

        let error = keyring
            .decrypt(&legacy_ciphertext)
            .expect_err("decrypt should fail");

        assert!(matches!(error, Error::DisabledKeyId { .. }));
    }

    #[test]
    fn duplicate_key_ids_are_rejected() {
        let duplicate_key_id = key_id("dup");
        let error = match Keyring::new(
            vec![
                KeyMaterial {
                    key_id: duplicate_key_id.clone(),
                    key: [1_u8; 32],
                },
                KeyMaterial {
                    key_id: duplicate_key_id.clone(),
                    key: [2_u8; 32],
                },
            ],
            duplicate_key_id,
            &[],
        ) {
            Ok(_) => panic!("keyring should fail"),
            Err(error) => error,
        };

        assert!(matches!(error, Error::DuplicateKeyId { .. }));
    }

    #[test]
    fn invalid_nonce_length_fails_closed() {
        let keyring = keyring(&[]);
        let error = keyring
            .decrypt(&SealedValue {
                key_id: key_id("active_data_key"),
                nonce: vec![1, 2, 3],
                ciphertext: vec![4, 5, 6],
            })
            .expect_err("decrypt should fail");

        assert!(matches!(error, Error::InvalidNonceLength { len: 3 }));
    }

    #[test]
    fn unknown_disabled_key_id_is_rejected() {
        let error = Keyring::new(
            vec![KeyMaterial {
                key_id: key_id("active_data_key"),
                key: [8_u8; 32],
            }],
            key_id("active_data_key"),
            &[key_id("missing_key")],
        )
        .err()
        .expect("keyring should fail");

        assert!(matches!(error, Error::UnknownDisabledKeyId { .. }));
    }
}
