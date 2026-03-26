//! Threshold signature aggregation.

use serde::{Deserialize, Serialize};

use lmprk_core::error::{LmprkError, Result};

use crate::ed25519::{verify_signature, SignerKey};