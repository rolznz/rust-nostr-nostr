// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

//! NIP94
//!
//! <https://github.com/nostr-protocol/nips/blob/master/94.md>

use alloc::string::String;
use alloc::vec::Vec;

use bitcoin::hashes::sha256::Hash as Sha256Hash;
use url_fork::Url;

use crate::{ImageDimensions, Tag};

/// Potential errors returned when parsing tags into a [FileMetadata] struct
#[derive(Debug, PartialEq, Eq)]
pub enum FileMetadataError {
    /// The URL of the file is missing (no `url` tag)
    MissingUrl,
    /// The mime type of the file is missing (no `m` tag)
    MissingMimeType,
    /// The SHA256 hash of the file is missing (no `x` tag)
    MissingSha,
}

impl core::fmt::Display for FileMetadataError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::MissingUrl => write!(f, "missing url"),
            Self::MissingMimeType => write!(f, "missing mime type"),
            Self::MissingSha => write!(f, "missing file sha256"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FileMetadataError {}

/// File Metadata
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMetadata {
    /// Url
    pub url: Url,
    /// MIME type
    pub mime_type: String,
    /// SHA256 of file
    pub hash: Sha256Hash,
    /// AES 256 GCM
    pub aes_256_gcm: Option<(String, String)>,
    /// Size in bytes
    pub size: Option<usize>,
    /// Size in pixels
    pub dim: Option<ImageDimensions>,
    /// Magnet
    pub magnet: Option<String>,
    /// Blurhash
    pub blurhash: Option<String>,
}

impl FileMetadata {
    /// New [`FileMetadata`]
    pub fn new<S>(url: Url, mime_type: S, hash: Sha256Hash) -> Self
    where
        S: Into<String>,
    {
        Self {
            url,
            mime_type: mime_type.into(),
            hash,
            aes_256_gcm: None,
            size: None,
            dim: None,
            magnet: None,
            blurhash: None,
        }
    }

    /// Add AES 256 GCM
    pub fn aes_256_gcm<S>(self, key: S, iv: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            aes_256_gcm: Some((key.into(), iv.into())),
            ..self
        }
    }

    /// Add file size (bytes)
    pub fn size(self, size: usize) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    /// Add file size (pixels)
    pub fn dimensions(self, dim: ImageDimensions) -> Self {
        Self {
            dim: Some(dim),
            ..self
        }
    }

    /// Add magnet
    pub fn magnet<S>(self, magnet: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            magnet: Some(magnet.into()),
            ..self
        }
    }

    /// Add blurhash
    pub fn blurhash<S>(self, blurhash: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            blurhash: Some(blurhash.into()),
            ..self
        }
    }
}

impl From<FileMetadata> for Vec<Tag> {
    fn from(metadata: FileMetadata) -> Self {
        let mut tags = Vec::new();

        let FileMetadata {
            url,
            mime_type,
            hash,
            aes_256_gcm,
            size,
            dim,
            magnet,
            blurhash,
        } = metadata;

        tags.push(Tag::Url(url));
        tags.push(Tag::MimeType(mime_type));
        tags.push(Tag::Sha256(hash));

        if let Some((key, iv)) = aes_256_gcm {
            tags.push(Tag::Aes256Gcm { key, iv });
        }

        if let Some(size) = size {
            tags.push(Tag::Size(size));
        }

        if let Some(dim) = dim {
            tags.push(Tag::Dim(dim));
        }

        if let Some(magnet) = magnet {
            tags.push(Tag::Magnet(magnet));
        }

        if let Some(blurhash) = blurhash {
            tags.push(Tag::Blurhash(blurhash));
        }

        tags
    }
}

impl TryFrom<Vec<Tag>> for FileMetadata {
    type Error = FileMetadataError;

    fn try_from(value: Vec<Tag>) -> Result<Self, Self::Error> {
        let url = match value.iter().find(|t| matches!(t, Tag::Url(_))) {
            Some(Tag::Url(url)) => Ok(url),
            _ => Err(Self::Error::MissingUrl),
        }?;
        let mime = match value.iter().find(|t| matches!(t, Tag::MimeType(_))) {
            Some(Tag::MimeType(mime)) => Ok(mime),
            _ => Err(Self::Error::MissingMimeType),
        }?;
        let sha256 = match value.iter().find(|t| matches!(t, Tag::Sha256(_))) {
            Some(Tag::Sha256(sha256)) => Ok(sha256),
            _ => Err(Self::Error::MissingSha),
        }?;
        let mut metadata = FileMetadata::new(url.clone(), mime, *sha256);

        if let Some(Tag::Aes256Gcm { key, iv }) =
            value.iter().find(|t| matches!(t, Tag::Aes256Gcm { .. }))
        {
            metadata = metadata.aes_256_gcm(key, iv);
        }

        if let Some(Tag::Size(size)) = value.iter().find(|t| matches!(t, Tag::Size(_))) {
            metadata = metadata.size(*size);
        }

        if let Some(Tag::Dim(dim)) = value.iter().find(|t| matches!(t, Tag::Dim(_))) {
            metadata = metadata.dimensions(*dim);
        }

        if let Some(Tag::Magnet(magnet)) = value.iter().find(|t| matches!(t, Tag::Magnet(_))) {
            metadata = metadata.magnet(magnet);
        }

        if let Some(Tag::Blurhash(bh)) = value.iter().find(|t| matches!(t, Tag::Blurhash(_))) {
            metadata = metadata.blurhash(bh);
        }

        Ok(metadata)
    }
}

#[cfg(test)]
mod test {
    use core::str::FromStr;

    use super::*;
    use crate::{ImageDimensions, Tag};

    const IMAGE_URL: &str = "https://image.nostr.build/99a95fcb4b7a2591ad32467032c52a62d90a204d3b176bc2459ad7427a3f2b89.jpg";
    const IMAGE_HASH: &str = "1aea8e98e0e5d969b7124f553b88dfae47d1f00472ea8c0dbf4ac4577d39ef02";

    #[test]
    fn parses_valid_tag_vector() {
        let url = Url::parse(IMAGE_URL).unwrap();
        let hash = Sha256Hash::from_str(IMAGE_HASH).unwrap();
        let dim = ImageDimensions {
            width: 640,
            height: 640,
        };
        let tags = vec![
            Tag::Dim(dim.clone()),
            Tag::Sha256(hash),
            Tag::Url(url.clone()),
            Tag::MimeType(String::from("image/jpeg")),
        ];
        let got = FileMetadata::try_from(tags).unwrap();
        let expected = FileMetadata::new(url, "image/jpeg", hash).dimensions(dim);

        assert_eq!(expected, got);
    }

    #[test]
    fn returns_error_with_url_missing() {
        let hash = Sha256Hash::from_str(IMAGE_HASH).unwrap();
        let dim = ImageDimensions {
            width: 640,
            height: 640,
        };
        let tags = vec![
            Tag::Dim(dim.clone()),
            Tag::Sha256(hash),
            Tag::MimeType(String::from("image/jpeg")),
        ];
        let got = FileMetadata::try_from(tags).unwrap_err();

        assert_eq!(FileMetadataError::MissingUrl, got);
    }

    #[test]
    fn returns_error_with_mime_type_missing() {
        let url = Url::parse(IMAGE_URL).unwrap();
        let hash = Sha256Hash::from_str(IMAGE_HASH).unwrap();
        let dim = ImageDimensions {
            width: 640,
            height: 640,
        };
        let tags = vec![
            Tag::Dim(dim.clone()),
            Tag::Sha256(hash),
            Tag::Url(url.clone()),
        ];
        let got = FileMetadata::try_from(tags).unwrap_err();

        assert_eq!(FileMetadataError::MissingMimeType, got);
    }

    #[test]
    fn returns_error_with_sha_missing() {
        let url = Url::parse(IMAGE_URL).unwrap();
        let dim = ImageDimensions {
            width: 640,
            height: 640,
        };
        let tags = vec![
            Tag::Dim(dim.clone()),
            Tag::Url(url.clone()),
            Tag::MimeType(String::from("image/jpeg")),
        ];
        let got = FileMetadata::try_from(tags).unwrap_err();

        assert_eq!(FileMetadataError::MissingSha, got);
    }
}
