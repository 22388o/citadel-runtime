// Citadel: Bitcoin, LN & RGB wallet runtime
// Written in 2021 by
//     Dr. Maxim Orlovsky <orlovsky@mycitadel.io>
//
// To the extent possible under law, the author(s) have dedicated all
// copyright and related and neighboring rights to this software to
// the public domain worldwide. This software is distributed without
// any warranty.
//
// You should have received a copy of the AGPL License
// along with this software.
// If not, see <https://www.gnu.org/licenses/agpl-3.0-standalone.html>.

use chrono::NaiveDateTime;
use serde_with::DisplayFromStr;
use std::collections::{BTreeMap, BTreeSet, HashSet};

use bitcoin::{Address, BlockHash, OutPoint, Transaction, Txid};
use wallet::bip32::UnhardenedIndex;

use crate::model::{ContractId, Utxo};

#[serde_as]
#[derive(
    Serialize,
    Deserialize,
    Getters,
    Clone,
    PartialEq,
    Debug,
    Default,
    StrictEncode,
    StrictDecode,
)]
pub(super) struct Cache {
    pub known_height: u32,

    #[serde_as(as = "BTreeMap<DisplayFromStr, _>")]
    pub descriptors: BTreeMap<ContractId, ContractCache>,

    #[serde_as(as = "Vec<(DisplayFromStr, _)>")]
    pub block_info: Vec<(BlockHash, NaiveDateTime)>,

    /// Mapping transaction id to the block height and block offset
    #[serde_as(as = "BTreeMap<(_, _), DisplayFromStr>")]
    pub mine_info: BTreeMap<(u32, u16), Txid>,

    #[serde_as(as = "BTreeMap<DisplayFromStr, _>")]
    pub tx_cache: BTreeMap<Txid, Transaction>,
}

#[serde_as]
#[derive(
    Serialize,
    Deserialize,
    Clone,
    PartialEq,
    Debug,
    Default,
    StrictEncode,
    StrictDecode,
)]
pub(super) struct ContractCache {
    pub updated_height: u32,

    pub used_address_derivations: BTreeMap<Address, UnhardenedIndex>,

    #[serde_as(as = "BTreeSet<DisplayFromStr>")]
    pub utxo: BTreeSet<OutPoint>,

    #[serde_as(as = "BTreeMap<DisplayFromStr, HashSet<_>>")]
    pub unspent: BTreeMap<rgb::ContractId, HashSet<Utxo>>,
}
