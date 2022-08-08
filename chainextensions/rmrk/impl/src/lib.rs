#![cfg_attr(not(feature = "std"), no_std)]

use sp_runtime::{traits::StaticLookup, DispatchError, Permill};

use chain_extension_traits::ChainExtensionExec;

use codec::Encode;
use frame_support::BoundedVec;
use frame_system::RawOrigin;
use pallet_contracts::chain_extension::{Environment, Ext, InitState, SysConfig, UncheckedFrom};
use pallet_rmrk_core::BoundedResourceTypeOf;
use rmrk_chain_extension_types::RmrkFunc;
use rmrk_traits::{
	primitives::{BaseId, CollectionId, NftId, PartId, ResourceId, SlotId},
	AccountIdOrCollectionNftTuple, BasicResource, ComposableResource, SlotResource,
};
use sp_std::{marker::PhantomData, vec::Vec};

pub struct RmrkExtension<R>(PhantomData<R>);

impl<
		T: pallet_rmrk_core::Config
			+ pallet_uniques::Config<CollectionId = CollectionId, ItemId = NftId>,
	> ChainExtensionExec<T> for RmrkExtension<T>
{
	fn execute_func<E>(func_id: u32, env: Environment<E, InitState>) -> Result<(), DispatchError>
	where
		E: Ext<T = T>,
		<E::T as SysConfig>::AccountId: UncheckedFrom<<E::T as SysConfig>::Hash> + AsRef<[u8]>,
	{
		let func_id = RmrkFunc::try_from(func_id)?;

		match func_id {
			RmrkFunc::NextNftId => {
				let mut env = env.buf_in_buf_out();
				let collection_id: u32 = env.read_as()?;

				let nft_id = pallet_rmrk_core::Pallet::<T>::next_nft_id(collection_id);
				let nft_id_encoded = nft_id.encode();

				env.write(&nft_id_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write next_nft_id")
				})?;
			},

			RmrkFunc::CollectionIndex => {
				let mut env = env.buf_in_buf_out();
				let index = pallet_rmrk_core::Pallet::<T>::collection_index();
				let index_encoded = index.encode();

				env.write(&index_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write collection_index")
				})?;
			},

			RmrkFunc::NextResourceId => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id): (T::CollectionId, T::ItemId) = env.read_as()?;

				let resource_id =
					pallet_rmrk_core::Pallet::<T>::next_resource_id(collection_id, nft_id);
				let resource_id_encoded = resource_id.encode();

				env.write(&resource_id_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write next_resource_id")
				})?;
			},

			RmrkFunc::Collections => {
				let mut env = env.buf_in_buf_out();
				let collection_id: T::CollectionId = env.read_as()?;

				let collections = pallet_rmrk_core::Pallet::<T>::collections(collection_id);

				let collections_encoded = collections.encode();

				env.write(&collections_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write collections_encoded")
				})?;
			},

			RmrkFunc::Nfts => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id): (T::CollectionId, T::ItemId) = env.read_as()?;

				let nfts = pallet_rmrk_core::Pallet::<T>::nfts(collection_id, nft_id);
				let nfts_encoded = nfts.encode();

				env.write(&nfts_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write nfts")
				})?;
			},

			RmrkFunc::Priorities => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource_id): (T::CollectionId, T::ItemId, ResourceId) =
					env.read_as()?;

				let priorities =
					pallet_rmrk_core::Pallet::<T>::priorities((collection_id, nft_id, resource_id));
				let priorities_encoded = priorities.encode();

				env.write(&priorities_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write priorities_encoded")
				})?;
			},

			RmrkFunc::Children => {
				let mut env = env.buf_in_buf_out();
				let ((parent_collection_id, parent_nft_id), (child_collection_id, child_nft_id)): (
					(T::CollectionId, T::ItemId),
					(T::CollectionId, T::ItemId),
				) = env.read_as()?;

				let children_res = pallet_rmrk_core::Pallet::<T>::children(
					(parent_collection_id, parent_nft_id),
					(child_collection_id, child_nft_id),
				);
				let children_res_encoded = children_res.encode();

				env.write(&children_res_encoded, false, None).map_err(|_| {
					DispatchError::Other(
						"RMRK chain Extension failed to write children_res_encoded",
					)
				})?;
			},

			RmrkFunc::Resources => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource_id): (T::CollectionId, T::ItemId, ResourceId) =
					env.read_as()?;

				let resources =
					pallet_rmrk_core::Pallet::<T>::resources((collection_id, nft_id, resource_id));
				let resources_encoded = resources.encode();

				env.write(&resources_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write resources_encoded")
				})?;
			},

			RmrkFunc::EquippableBases => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, base_id): (T::CollectionId, T::ItemId, BaseId) =
					env.read_as()?;

				let equippable_base_res = pallet_rmrk_core::Pallet::<T>::equippable_bases((
					collection_id,
					nft_id,
					base_id,
				));
				let equippable_base_res_encoded = equippable_base_res.encode();

				env.write(&equippable_base_res_encoded, false, None).map_err(|_| {
					DispatchError::Other(
						"RMRK chain Extension failed to write equippable_base_res_encoded",
					)
				})?;
			},

			RmrkFunc::EquippableSlots => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource_id, base_id, slot_id): (
					T::CollectionId,
					T::ItemId,
					ResourceId,
					BaseId,
					SlotId,
				) = env.read_as()?;

				let equippable_slot_res = pallet_rmrk_core::Pallet::<T>::equippable_slots((
					collection_id,
					nft_id,
					resource_id,
					base_id,
					slot_id,
				));
				let equippable_slot_res_encoded = equippable_slot_res.encode();

				env.write(&equippable_slot_res_encoded, false, None).map_err(|_| {
					DispatchError::Other(
						"RMRK chain Extension failed to write equippable_slot_res_encoded",
					)
				})?;
			},

			RmrkFunc::Properties => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, maybe_nft_id, key): (
					T::CollectionId,
					Option<T::ItemId>,
					BoundedVec<u8, T::KeyLimit>,
				) = env.read_as_unbounded(env.in_len())?;

				let properties =
					pallet_rmrk_core::Pallet::<T>::properties((collection_id, maybe_nft_id, key));
				let properties_encoded = properties.encode();

				env.write(&properties_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write properties_encoded")
				})?;
			},

			RmrkFunc::Lock => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id): (T::CollectionId, T::ItemId) = env.read_as()?;

				let lock = pallet_rmrk_core::Pallet::<T>::lock((collection_id, nft_id));
				let lock_encoded = lock.encode();

				env.write(&lock_encoded, false, None).map_err(|_| {
					DispatchError::Other("RMRK chain Extension failed to write lock")
				})?;
			},

			RmrkFunc::MintNft => {
				let mut env = env.buf_in_buf_out();
				let (
					owner,
					collection_id,
					royalty_recipient,
					royalty,
					metadata,
					transferable,
					resources,
				): (
					T::AccountId,
					T::CollectionId,
					Option<T::AccountId>,
					Option<Permill>,
					Vec<u8>,
					bool,
					Option<BoundedResourceTypeOf<T>>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::mint_nft(
					RawOrigin::Signed(caller_contract).into(),
					Some(owner.clone()),
					collection_id,
					royalty_recipient,
					royalty,
					metadata.try_into().unwrap(),
					transferable,
					resources,
				)?;
			},

			RmrkFunc::MintNftDirectlyToNft => {
				let mut env = env.buf_in_buf_out();
				let (
					owner,
					collection_id,
					royalty_recipient,
					royalty,
					metadata,
					transferable,
					resources,
				): (
					(T::CollectionId, T::ItemId),
					T::CollectionId,
					Option<T::AccountId>,
					Option<Permill>,
					BoundedVec<u8, T::StringLimit>,
					bool,
					Option<BoundedResourceTypeOf<T>>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::mint_nft_directly_to_nft(
					RawOrigin::Signed(caller_contract).into(),
					owner,
					collection_id,
					royalty_recipient,
					royalty,
					metadata.try_into().unwrap(),
					transferable,
					resources,
				)?;
			},

			RmrkFunc::CreateCollection => {
				let mut env = env.buf_in_buf_out();
				let (metadata, max, symbol): (Vec<u8>, Option<u32>, Vec<u8>) =
					env.read_as_unbounded(env.in_len())?;

				let caller_contract = env.ext().address().clone();

				let weight = 100_000_000_000; // TODO update after RMRK pallet implements weights
				env.charge_weight(weight)?;

				sp_std::if_std! {println!(
					"[RmrkExtension] create_collection metadata{:?}, symbol{:?}, caller{:?}, weight {:?}",
					metadata, symbol, caller_contract, weight
				);}
				let create_result = pallet_rmrk_core::Pallet::<T>::create_collection(
					RawOrigin::Signed(caller_contract).into(),
					metadata.try_into().unwrap(),
					max,
					symbol.try_into().unwrap(),
				);
				sp_std::if_std! {println!(
					"[RmrkExtension] create_result {:?}",
					create_result
				);}
			},

			RmrkFunc::BurnNft => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, max_burns): (T::CollectionId, T::ItemId, u32) =
					env.read_as()?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::burn_nft(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
					max_burns,
				)?;
			},

			RmrkFunc::DestroyCollection => {
				let mut env = env.buf_in_buf_out();
				let collection_id: u32 = env.read_as()?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::destroy_collection(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
				)?;
			},

			RmrkFunc::Send => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, new_owner): (
					T::CollectionId,
					T::ItemId,
					AccountIdOrCollectionNftTuple<T::AccountId>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::send(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
					new_owner,
				)?;
			},

			RmrkFunc::AcceptNft => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, new_owner): (
					T::CollectionId,
					T::ItemId,
					AccountIdOrCollectionNftTuple<T::AccountId>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::accept_nft(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
					new_owner,
				)?;
			},

			RmrkFunc::RejectNft => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id): (T::CollectionId, T::ItemId) = env.read_as()?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::reject_nft(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
				)?;
			},

			RmrkFunc::ChangeCollectionIssuer => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, new_issuer): (T::CollectionId, T::AccountId) = env.read_as()?;

				let new_issuer = <T::Lookup as StaticLookup>::unlookup(new_issuer);

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::change_collection_issuer(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					new_issuer,
				)?;
			},

			RmrkFunc::SetProperty => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, maybe_nft_id, key, value): (
					T::CollectionId,
					Option<T::ItemId>,
					BoundedVec<u8, T::KeyLimit>,
					BoundedVec<u8, T::ValueLimit>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::set_property(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					maybe_nft_id,
					key,
					value,
				)?;
			},

			RmrkFunc::LockCollection => {
				let mut env = env.buf_in_buf_out();
				let collection_id: u32 = env.read_as()?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::lock_collection(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
				)?;
			},

			RmrkFunc::AddBasicResource => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource): (
					T::CollectionId,
					T::ItemId,
					BasicResource<BoundedVec<u8, T::StringLimit>>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::add_basic_resource(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
					resource,
				)?;
			},

			RmrkFunc::AddComposableResource => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource): (
					T::CollectionId,
					T::ItemId,
					ComposableResource<
						BoundedVec<u8, T::StringLimit>,
						BoundedVec<PartId, T::PartsLimit>,
					>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::add_composable_resource(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
					resource,
				)?;
			},

			RmrkFunc::AddSlotResource => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource): (
					T::CollectionId,
					T::ItemId,
					SlotResource<BoundedVec<u8, T::StringLimit>>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::add_slot_resource(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
					resource,
				)?;
			},

			RmrkFunc::AcceptResource => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource_id): (T::CollectionId, T::ItemId, ResourceId) =
					env.read_as()?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::accept_resource(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
					resource_id,
				)?;
			},

			RmrkFunc::RemoveResource => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource_id): (T::CollectionId, T::ItemId, ResourceId) =
					env.read_as()?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::remove_resource(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
					resource_id,
				)?;
			},

			RmrkFunc::AcceptResourceRemoval => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, resource_id): (T::CollectionId, T::ItemId, ResourceId) =
					env.read_as()?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::accept_resource_removal(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
					resource_id,
				)?;
			},

			RmrkFunc::SetPriority => {
				let mut env = env.buf_in_buf_out();
				let (collection_id, nft_id, priorities): (
					T::CollectionId,
					T::ItemId,
					BoundedVec<ResourceId, T::MaxPriorities>,
				) = env.read_as_unbounded(env.in_len())?;

				let caller_contract = env.ext().address().clone();
				pallet_rmrk_core::Pallet::<T>::set_priority(
					RawOrigin::Signed(caller_contract).into(),
					collection_id,
					nft_id,
					priorities,
				)?;
			},
		}

		Ok(())
	}
}
