use crate::mock::{new_tester, AccountId, Origin, Event, System, Balances, DOT};
use frame_support::{assert_ok, traits::Currency};
use sp_keyring::AccountKeyring as Keyring;
use sp_core::H160;
use crate::RawEvent;

use artemis_core::ChannelId;

fn last_event() -> Event {
	System::events().pop().expect("Event expected").event
}


#[test]
fn should_lock() {
	new_tester().execute_with(|| {
		let sender: AccountId = Keyring::Bob.into();
		let recipient = H160::repeat_byte(2);
		let amount = 100;

		let _ = Balances::deposit_creating(&sender, amount * 2);

		assert_ok!(DOT::lock(
			Origin::signed(sender.clone()),
			ChannelId::Incentivized,
			recipient.clone(),
			amount));

		assert_eq!(Balances::total_balance(&DOT::account_id()), amount);

		assert_eq!(
			Event::dot_app(RawEvent::Locked(sender, recipient, amount)),
			last_event()
		);
	});
}

#[test]
fn should_unlock() {
	new_tester().execute_with(|| {
		let peer_contract = H160::repeat_byte(1);
		let sender = H160::repeat_byte(7);
		let recipient: AccountId = Keyring::Bob.into();
		let amount = 100;
		let balance = 500;

		let _ = Balances::deposit_creating(&DOT::account_id(), balance);

		assert_ok!(
			DOT::unlock(
				artemis_dispatch::Origin(peer_contract).into(),
				sender,
				recipient.clone(),
				amount
			)
		);
		assert_eq!(Balances::total_balance(&recipient), amount);
		assert_eq!(Balances::total_balance(&DOT::account_id()), balance - amount);

		assert_eq!(
			Event::dot_app(RawEvent::Unlocked(sender, recipient, amount.into())),
			last_event()
		);
	});
}

