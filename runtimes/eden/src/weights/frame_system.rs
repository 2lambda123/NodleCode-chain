/*
 * This file is part of the Nodle Chain distributed at https://github.com/NodleCode/chain
 * Copyright (C) 2020-2022  Nodle International
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

//! Autogenerated weights for frame_system
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2022-10-19, STEPS: `50`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/nodle-parachain
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=*
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --template=./.maintain/frame-weight-template.hbs
// --output=weights

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for frame_system.
pub trait WeightInfo {
	fn remark(b: u32) -> Weight;
	fn remark_with_event(b: u32) -> Weight;
	fn set_heap_pages() -> Weight;
	fn set_storage(i: u32) -> Weight;
	fn kill_storage(i: u32) -> Weight;
	fn kill_prefix(p: u32) -> Weight;
}

/// Weights for frame_system using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn remark(_b: u32) -> Weight {
		Weight::from_ref_time(6_116_000 as u64)
	}
	fn remark_with_event(b: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn set_heap_pages() -> Weight {
		Weight::from_ref_time(19_440_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	fn set_storage(i: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_911_000).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	fn kill_storage(i: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_482_000).saturating_mul(i as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	fn kill_prefix(p: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 62_000
			.saturating_add(Weight::from_ref_time(2_709_000).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn remark(_b: u32) -> Weight {
		Weight::from_ref_time(6_116_000 as u64)
	}
	fn remark_with_event(b: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 0
			.saturating_add(Weight::from_ref_time(2_000).saturating_mul(b as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn set_heap_pages() -> Weight {
		Weight::from_ref_time(19_440_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	fn set_storage(i: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_911_000).saturating_mul(i as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	fn kill_storage(i: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 2_000
			.saturating_add(Weight::from_ref_time(1_482_000).saturating_mul(i as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(i as u64)))
	}
	fn kill_prefix(p: u32) -> Weight {
		Weight::from_ref_time(0 as u64)
			// Standard Error: 62_000
			.saturating_add(Weight::from_ref_time(2_709_000).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().writes((1 as u64).saturating_mul(p as u64)))
	}
}