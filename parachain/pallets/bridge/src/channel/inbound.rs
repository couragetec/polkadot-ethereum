use frame_support::{dispatch::{DispatchError, DispatchResult}, storage::StorageMap};
use sp_std::{cell::Cell, marker::PhantomData, boxed::Box};
use sp_core::RuntimeDebug;
use artemis_core::{ChannelId, MessageDispatch};
use crate::{
	Config, Error, InboundChannels,
	envelope::Envelope, primitives::{InboundChannel, InboundChannelData}
};

use codec::{Encode, Decode};

/// Construct an inbound channel object
pub fn make_inbound_channel<T>(channel_id: ChannelId) -> Box<dyn InboundChannel<T::AccountId>>
where
	T: Config
{
	match channel_id {
		ChannelId::Basic => Box::new(BasicInboundChannel::<T>::new()),
		ChannelId::Incentivized => Box::new(IncentivizedInboundChannel::<T>::new()),
	}
}


struct BasicInboundChannel<T: Config> {
	channel_id: ChannelId,
	storage: Storage<T>
}

impl<T: Config> BasicInboundChannel<T> {
	fn new() -> Self {
		Self {
			channel_id: ChannelId::Basic,
			storage: Storage::new(ChannelId::Basic)
		}
	}
}

impl<T: Config> InboundChannel<T::AccountId> for BasicInboundChannel<T> {
	fn submit(&self, relayer: &T::AccountId, envelope: &Envelope) -> DispatchResult {
		self.storage.try_mutate::<_,DispatchError,_>(|data| {
			if envelope.nonce != data.nonce {
				return Err(Error::<T>::BadNonce.into())
			}
			data.nonce += 1;
			Ok(())
		})?;

		let message_id = (self.channel_id, envelope.nonce);
		T::MessageDispatch::dispatch(envelope.source, message_id, &envelope.payload);

		Ok(())
	}
}

/// Incentivized Channel
struct IncentivizedInboundChannel<T: Config> {
	channel_id: ChannelId,
	storage: Storage<T>
}

impl<T: Config> IncentivizedInboundChannel<T> {
	fn new() -> Self {
		Self {
			channel_id: ChannelId::Incentivized,
			storage: Storage::new(ChannelId::Incentivized)
		}
	}
}

impl<T: Config> InboundChannel<T::AccountId> for IncentivizedInboundChannel<T> {
	fn submit(&self, relayer: &T::AccountId, envelope: &Envelope) -> DispatchResult {
		self.storage.try_mutate::<_,DispatchError,_>(|data| {
			if envelope.nonce != data.nonce {
				return Err(Error::<T>::BadNonce.into())
			}
			data.nonce += 1;
			Ok(())
		})?;

		//let metadata = Metadata::decode(&mut envelope.metadata.as_ref()).map_err(|_| ())?;


		let message_id = (self.channel_id, envelope.nonce);
		T::MessageDispatch::dispatch(envelope.source, message_id, &envelope.payload);

		Ok(())
	}
}


#[derive(Encode, Decode, Copy, Clone, PartialEq, Eq, RuntimeDebug)]
struct Metadata {
	fee: u128
}


struct Storage<T: Config> {
	channel_id: ChannelId,
	cached_data: Cell<Option<InboundChannelData>>,
	phantom: PhantomData<T>
}

impl<T: Config> Storage<T> {
	fn new(channel_id: ChannelId) -> Self {
		Storage {
			channel_id,
			cached_data: Cell::new(None),
			phantom: PhantomData
		}
	}

	#[allow(dead_code)]
	fn get(&self) -> InboundChannelData {
		match self.cached_data.get() {
			Some(data) => data,
			None => {
				let data = InboundChannels::get(self.channel_id);
				self.cached_data.set(Some(data));
				data
			}
		}
	}

	#[allow(dead_code)]
	fn set(&self, data: InboundChannelData) {
		self.cached_data.set(Some(data));
		InboundChannels::insert(self.channel_id, data)
	}

	#[allow(dead_code)]
	fn try_mutate<R, E, F>(&self, f: F) -> Result<R, E>
	where
		F: FnOnce(&mut InboundChannelData) -> Result<R, E>
	{
		let mut data = self.get();
		let result = f(&mut data);
		if result.is_ok() {
			self.set(data);
		}
		result
	}
}
