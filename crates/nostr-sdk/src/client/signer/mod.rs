// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! Client Signers

use std::fmt;

#[cfg(all(feature = "nip07", target_arch = "wasm32"))]
use nostr::nips::nip07::Nip07Signer;
use nostr::Keys;

#[cfg(feature = "nip46")]
pub mod nip46;

#[cfg(feature = "nip46")]
use self::nip46::Nip46Signer;
#[cfg(feature = "nip46")]
use super::Error;

/// Client Signer Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ClientSignerType {
    /// Keys
    Keys,
    /// NIP07
    #[cfg(all(feature = "nip07", target_arch = "wasm32"))]
    NIP07,
    /// NIP46
    #[cfg(feature = "nip46")]
    NIP46,
}

// TODO: better display
impl fmt::Display for ClientSignerType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Keys => write!(f, "Keys"),
            #[cfg(all(feature = "nip07", target_arch = "wasm32"))]
            Self::NIP07 => write!(f, "NIP07"),
            #[cfg(feature = "nip46")]
            Self::NIP46 => write!(f, "NIP46"),
        }
    }
}

/// Client signer
#[derive(Debug, Clone)]
pub enum ClientSigner {
    /// Private Keys
    Keys(Keys),
    /// NIP07 signer
    #[cfg(all(feature = "nip07", target_arch = "wasm32"))]
    NIP07(Nip07Signer),
    /// NIP46 signer
    #[cfg(feature = "nip46")]
    NIP46(Nip46Signer),
}

impl ClientSigner {
    /// Get Client Signer Type
    pub fn r#type(&self) -> ClientSignerType {
        match self {
            Self::Keys(..) => ClientSignerType::Keys,
            #[cfg(all(feature = "nip07", target_arch = "wasm32"))]
            Self::NIP07(..) => ClientSignerType::NIP07,
            #[cfg(feature = "nip46")]
            Self::NIP46(..) => ClientSignerType::NIP46,
        }
    }
}

impl From<Keys> for ClientSigner {
    fn from(keys: Keys) -> Self {
        Self::Keys(keys)
    }
}

impl From<&Keys> for ClientSigner {
    fn from(keys: &Keys) -> Self {
        Self::Keys(keys.clone())
    }
}

#[cfg(all(feature = "nip07", target_arch = "wasm32"))]
impl From<Nip07Signer> for ClientSigner {
    fn from(nip07: Nip07Signer) -> Self {
        Self::NIP07(nip07)
    }
}

#[cfg(feature = "nip46")]
impl From<Nip46Signer> for ClientSigner {
    fn from(nip46: Nip46Signer) -> Self {
        Self::NIP46(nip46)
    }
}

#[cfg(feature = "nip46")]
impl TryFrom<ClientSigner> for Nip46Signer {
    type Error = Error;

    fn try_from(signer: ClientSigner) -> Result<Self, Self::Error> {
        if let ClientSigner::NIP46(nip46) = signer {
            Ok(nip46)
        } else {
            Err(Error::WrongSigner {
                expected: ClientSignerType::NIP46,
                found: signer.r#type(),
            })
        }
    }
}
