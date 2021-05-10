/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020  Nodle International
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

use frame_support::{
    migration::{put_storage_value, StorageIterator},
    traits::OnRuntimeUpgrade,
    weights::{constants::RocksDbWeight, Weight},
};
use nodle_chain_primitives::{Balance, BlockNumber};
use pallet_grants::VestingSchedule;
use sp_runtime::traits::Saturating;
use sp_std::vec::Vec;

/// This migration is in charge of updating the grants to work with the last chain reboot.
/// We have to do two things:
/// - due to the sub2 to sub3 upgrade the storage hashes are not the same, we need
///   to switch our grants from the "Vesting" name to "Grants"
/// - due to the chain reboot to readjust the grant block numbers
pub struct GrantsMigration;
impl OnRuntimeUpgrade for GrantsMigration {
    fn on_runtime_upgrade() -> Weight {
        // Buffer to account for misc checks
        let mut weight: Weight = 1_000;

        sp_runtime::print("🕊️ Starting grants migration...");

        for (account_id, grants) in
            StorageIterator::<Vec<VestingSchedule<BlockNumber, Balance>>>::new(
                b"Vesting",
                b"VestingSchedules",
            )
            .drain()
        {
            // The network was stopped at block 2_756_825. We simply remove those blocks from
            // the start value since the network restarts at block 0.
            let previous_network_stopped_at = 2_756_825;
            put_storage_value(
                b"Grants", // Note how we are switching from Vesting to Grants
                b"VestingSchedules",
                &account_id,
                grants
                    .iter()
                    .clone()
                    .map(|grant| VestingSchedule::<BlockNumber, Balance> {
                        start: grant.start.saturating_sub(previous_network_stopped_at),
                        period: grant.period,
                        period_count: grant.period_count,
                        per_period: grant.per_period,
                    })
                    .collect::<Vec<_>>(),
            );
            weight = weight.saturating_add(RocksDbWeight::get().reads_writes(1, 1));
        }

        sp_runtime::print("🕊️ Grants migration done...");

        weight
    }
}
