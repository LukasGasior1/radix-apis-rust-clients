/*
 * Radix Gateway API - Babylon
 *
 * This API is exposed by the Babylon Radix Gateway to enable clients to efficiently query current and historic state on the RadixDLT ledger, and intelligently handle transaction submission.  It is designed for use by wallets and explorers, and for light queries from front-end dApps. For exchange/asset integrations, back-end dApp integrations, or simple use cases, you should consider using the Core API on a Node. A Gateway is only needed for reading historic snapshots of ledger states or a more robust set-up.  The Gateway API is implemented by the [Network Gateway](https://github.com/radixdlt/babylon-gateway), which is configured to read from [full node(s)](https://github.com/radixdlt/babylon-node) to extract and index data from the network.  This document is an API reference documentation, visit [User Guide](https://docs.radixdlt.com/) to learn more about how to run a Gateway of your own.  ## Migration guide  Please see [the latest release notes](https://github.com/radixdlt/babylon-gateway/releases).  ## Integration and forward compatibility guarantees  All responses may have additional fields added at any release, so clients are advised to use JSON parsers which ignore unknown fields on JSON objects.  When the Radix protocol is updated, new functionality may be added, and so discriminated unions returned by the API may need to be updated to have new variants added, corresponding to the updated data. Clients may need to update in advance to be able to handle these new variants when a protocol update comes out.  On the very rare occasions we need to make breaking changes to the API, these will be warned in advance with deprecation notices on previous versions. These deprecation notices will include a safe migration path. Deprecation notes or breaking changes will be flagged clearly in release notes for new versions of the Gateway.  The Gateway DB schema is not subject to any compatibility guarantees, and may be changed at any release. DB changes will be flagged in the release notes so clients doing custom DB integrations can prepare. 
 *
 * The version of the OpenAPI document: v1.6.1
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

/// ProgrammaticScryptoSborValueKind : These are the Scrypto SBOR `ValueKind`s, but with `Bytes` added as an alias for `Vec`, to display such values as hex-encoded strings. 
/// These are the Scrypto SBOR `ValueKind`s, but with `Bytes` added as an alias for `Vec`, to display such values as hex-encoded strings. 
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum ProgrammaticScryptoSborValueKind {
    #[serde(rename = "Bool")]
    Bool,
    #[serde(rename = "I8")]
    I8,
    #[serde(rename = "I16")]
    I16,
    #[serde(rename = "I32")]
    I32,
    #[serde(rename = "I64")]
    I64,
    #[serde(rename = "I128")]
    I128,
    #[serde(rename = "U8")]
    U8,
    #[serde(rename = "U16")]
    U16,
    #[serde(rename = "U32")]
    U32,
    #[serde(rename = "U64")]
    U64,
    #[serde(rename = "U128")]
    U128,
    #[serde(rename = "String")]
    String,
    #[serde(rename = "Enum")]
    Enum,
    #[serde(rename = "Array")]
    Array,
    #[serde(rename = "Bytes")]
    Bytes,
    #[serde(rename = "Map")]
    Map,
    #[serde(rename = "Tuple")]
    Tuple,
    #[serde(rename = "Reference")]
    Reference,
    #[serde(rename = "Own")]
    Own,
    #[serde(rename = "Decimal")]
    Decimal,
    #[serde(rename = "PreciseDecimal")]
    PreciseDecimal,
    #[serde(rename = "NonFungibleLocalId")]
    NonFungibleLocalId,

}

impl std::fmt::Display for ProgrammaticScryptoSborValueKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::Bool => write!(f, "Bool"),
            Self::I8 => write!(f, "I8"),
            Self::I16 => write!(f, "I16"),
            Self::I32 => write!(f, "I32"),
            Self::I64 => write!(f, "I64"),
            Self::I128 => write!(f, "I128"),
            Self::U8 => write!(f, "U8"),
            Self::U16 => write!(f, "U16"),
            Self::U32 => write!(f, "U32"),
            Self::U64 => write!(f, "U64"),
            Self::U128 => write!(f, "U128"),
            Self::String => write!(f, "String"),
            Self::Enum => write!(f, "Enum"),
            Self::Array => write!(f, "Array"),
            Self::Bytes => write!(f, "Bytes"),
            Self::Map => write!(f, "Map"),
            Self::Tuple => write!(f, "Tuple"),
            Self::Reference => write!(f, "Reference"),
            Self::Own => write!(f, "Own"),
            Self::Decimal => write!(f, "Decimal"),
            Self::PreciseDecimal => write!(f, "PreciseDecimal"),
            Self::NonFungibleLocalId => write!(f, "NonFungibleLocalId"),
        }
    }
}

impl Default for ProgrammaticScryptoSborValueKind {
    fn default() -> ProgrammaticScryptoSborValueKind {
        Self::Bool
    }
}

