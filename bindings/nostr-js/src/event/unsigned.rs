// Copyright (c) 2022-2023 Yuki Kishimoto
// Copyright (c) 2023-2024 Rust Nostr Developers
// Distributed under the MIT software license

use std::ops::Deref;
use std::str::FromStr;

use js_sys::Array;
use nostr::bitcoin::secp256k1::schnorr::Signature;
use nostr::{JsonUtil, UnsignedEvent};
use wasm_bindgen::prelude::*;

use super::tag::{JsTag, JsTagArray};
use crate::error::{into_err, Result};
use crate::event::{JsEvent, JsEventId};
use crate::key::{JsKeys, JsPublicKey};
use crate::types::JsTimestamp;

#[wasm_bindgen(js_name = UnsignedEvent)]
pub struct JsUnsignedEvent {
    inner: UnsignedEvent,
}

impl From<UnsignedEvent> for JsUnsignedEvent {
    fn from(inner: UnsignedEvent) -> Self {
        Self { inner }
    }
}

impl From<JsUnsignedEvent> for UnsignedEvent {
    fn from(event: JsUnsignedEvent) -> Self {
        event.inner
    }
}

#[wasm_bindgen(js_class = UnsignedEvent)]
impl JsUnsignedEvent {
    #[wasm_bindgen(getter)]
    pub fn id(&self) -> JsEventId {
        self.inner.id.into()
    }

    #[wasm_bindgen(getter)]
    pub fn pubkey(&self) -> JsPublicKey {
        self.inner.pubkey.into()
    }

    #[wasm_bindgen(js_name = createdAt, getter)]
    pub fn created_at(&self) -> JsTimestamp {
        self.inner.created_at.into()
    }

    #[wasm_bindgen(getter)]
    pub fn kind(&self) -> f64 {
        self.inner.kind.as_f64()
    }

    #[wasm_bindgen(getter)]
    pub fn tags(&self) -> JsTagArray {
        self.inner
            .tags
            .iter()
            .cloned()
            .map(|t| {
                let e: JsTag = t.into();
                JsValue::from(e)
            })
            .collect::<Array>()
            .unchecked_into()
    }

    #[wasm_bindgen(getter)]
    pub fn content(&self) -> String {
        self.inner.content.clone()
    }

    #[wasm_bindgen(js_name = fromJson)]
    pub fn from_json(json: String) -> Result<JsUnsignedEvent> {
        Ok(Self {
            inner: UnsignedEvent::from_json(json).map_err(into_err)?,
        })
    }

    #[wasm_bindgen(js_name = asJson)]
    pub fn as_json(&self) -> String {
        self.inner.as_json()
    }

    /// Sign
    pub fn sign(self, keys: &JsKeys) -> Result<JsEvent> {
        Ok(self.inner.sign(keys.deref()).map_err(into_err)?.into())
    }

    /// Add signature
    pub fn add_signature(self, sig: String) -> Result<JsEvent> {
        let sig: Signature = Signature::from_str(&sig).map_err(into_err)?;
        Ok(self.inner.add_signature(sig).map_err(into_err)?.into())
    }
}
