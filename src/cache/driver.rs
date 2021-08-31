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

use std::collections::{BTreeMap, BTreeSet, HashSet};

use bitcoin::{Address, OutPoint, Txid};
use wallet::hd::UnhardenedIndex;

use super::Error;
use crate::model::{Allocations, ContractId, Utxo};

pub trait Driver {
    fn blockpos_to_txid(&self, height: u32, offset: u16) -> Option<Txid>;

    fn unspent(
        &self,
        contract_id: ContractId,
    ) -> Result<BTreeMap<rgb::ContractId, HashSet<Utxo>>, Error>;

    fn unspent_bitcoin_only(
        &self,
        contract_id: ContractId,
    ) -> Result<HashSet<Utxo>, Error>;

    fn allocations(
        &self,
        contract_id: ContractId,
    ) -> Result<Allocations, Error>;

    fn utxo(
        &self,
        contract_id: ContractId,
    ) -> Result<BTreeSet<OutPoint>, Error>;

    fn update(
        &mut self,
        contract_id: ContractId,
        mine_info: BTreeMap<(u32, u16), Txid>,
        updated_height: Option<u32>,
        utxo: BTreeSet<OutPoint>,
        unspent: BTreeMap<rgb::ContractId, Vec<Utxo>>,
    ) -> Result<(), Error>;

    fn used_address_derivations(
        &self,
        contract_id: ContractId,
    ) -> Result<BTreeMap<Address, UnhardenedIndex>, Error>;

    fn used_addresses(
        &self,
        contract_id: ContractId,
    ) -> Result<HashSet<Address>, Error>;

    fn used_derivations(
        &self,
        contract_id: ContractId,
    ) -> Result<HashSet<UnhardenedIndex>, Error>;

    fn next_unused_derivation(
        &self,
        contract_id: ContractId,
    ) -> Result<UnhardenedIndex, Error>;

    fn use_address_derivation(
        &mut self,
        contract_id: ContractId,
        address: Address,
        path: UnhardenedIndex,
    ) -> Result<bool, Error>;

    fn last_used_derivation(
        &self,
        contract_id: ContractId,
    ) -> Option<UnhardenedIndex>;

    fn forget_address(
        &mut self,
        contract_id: ContractId,
        address: &Address,
    ) -> Result<bool, Error>;

    fn address_derivation(
        &self,
        contract_id: ContractId,
        address: &Address,
    ) -> Option<UnhardenedIndex>;
}
