// Copyright 2019 The Exonum Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Timestamping transactions.

// Workaround for `failure` see https://github.com/rust-lang-nursery/failure/issues/223 and
// ECR-1771 for the details.
#![allow(bare_trait_objects)]

use std::borrow::Cow;

use exonum::{
    blockchain::{ ExecutionResult, Transaction, TransactionContext},
    crypto::{PublicKey, SecretKey},
    messages::{Message, RawTransaction, Signed},
};

use super::proto;
use crate::schema::Timestamp;

pub const TIMESTAMPING_SERVICE: u16 = 130;

/// Timestamping transaction.
#[derive(Serialize, Deserialize, Clone, Debug, ProtobufConvert)]
#[exonum(pb = "proto::TxTimestamp")]
pub struct TxTimestamp {
    /// Timestamp content.
    pub content: Timestamp,
}

/// Transaction group.
#[derive(Serialize, Deserialize, Clone, Debug, TransactionSet)]
pub enum TimeTransactions {
    /// A timestamp transaction.
    TxTimestamp(TxTimestamp),
}

impl TxTimestamp {
    #[doc(hidden)]
    pub fn sign(author: &PublicKey, content: Timestamp, key: &SecretKey) -> Signed<RawTransaction> {
        Message::sign_transaction(Self { content }, TIMESTAMPING_SERVICE, *author, key)
    }
}

impl Transaction for TxTimestamp {
    fn execute(&self, _context: TransactionContext) -> ExecutionResult {
        Ok(())
    }
}
