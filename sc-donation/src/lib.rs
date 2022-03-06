#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Donation
{
  #[init]
  fn init(
    &self,
    min_donation: BigUint,
    tier_thresholds: ManagedVec<BigUint>,
    max_donation_destination_id: DonationDestinationId,
    distribution_address: ManagedAddress,
    collection_id: TokenIdentifier,
  ) {
    self.min_donation().set(&min_donation);
    self.tier_thresholds().set(&tier_thresholds);
    self.max_donation_destination_id().set(&max_donation_destination_id);
    self.distribution_address().set(&distribution_address);
    self.collection().set_token_id(&collection_id);
  }

  #[payable("EGLD")]
  #[endpoint(donate)]
  fn donate(
    &self,
    #[payment_amount] donation: BigUint,
    pseudo: ManagedBuffer,
    twitter_handle: ManagedBuffer,
    message: ManagedBuffer,
    donation_destination_id: DonationDestinationId,
  ) {
    require!(self.donation_state().get() == DonationState::Active, "Donations not enabled");
    require!(donation >= self.min_donation().get(), "Donation too small");
    self.require_valid_pseudo(pseudo.clone());
    self.require_valid_twitter_handle(twitter_handle.clone());
    self.require_valid_message(message.clone());
    require!(donation_destination_id <= self.max_donation_destination_id().get(), "Invalid donation destination id");
    let donor_address = self.blockchain().get_caller();
    let donor_data =
      if self.donors_data(&donor_address).is_empty() {
        let num_donors = self.num_donors().get() + 1;
        self.num_donors().set(num_donors);
        DonorData {
          id: num_donors,
          pseudo: pseudo,
          twitter_handle: twitter_handle,
          message: message,
          donation_destination_id: donation_destination_id,
          donation: donation.clone(),
          last_minted_tier_id: 0,
        }
      } else {
        let old_donor_data = self.donors_data(&donor_address).get();
        DonorData {
          id: old_donor_data.id,
          pseudo: pseudo,
          twitter_handle: twitter_handle,
          message: message,
          donation_destination_id: donation_destination_id,
          donation: old_donor_data.donation + donation.clone(),
          last_minted_tier_id: old_donor_data.last_minted_tier_id,
        }
      };
    self.donors_data(&donor_address).set(donor_data);
    let total_donation = self.total_donation().get();
    self.total_donation().set(total_donation + donation.clone());
    self.donation_event(donor_address, donation);
  }

  fn require_valid_pseudo(&self, pseudo: ManagedBuffer) {
    require!(pseudo.len() <= PSEUDO_MAX_LENGTH, "Pseudo too long");
    let mut str_bytes = [0u8; PSEUDO_MAX_LENGTH];
    let s = &mut str_bytes[..pseudo.len()];
    require!(!pseudo.load_slice(0, s).is_err(), "Error loading pseudo bytes");
    for ch in s.iter() {
      require!(is_valid_pseudo_char(*ch), "Invalid character found in pseudo");
    }
  }

  fn require_valid_twitter_handle(&self, twitter_handle: ManagedBuffer) {
    require!(twitter_handle.len() <= TWITTER_HANDLE_MAX_LENGTH, "Twitter handle too long");
    let mut str_bytes = [0u8; TWITTER_HANDLE_MAX_LENGTH];
    let s = &mut str_bytes[..twitter_handle.len()];
    require!(!twitter_handle.load_slice(0, s).is_err(), "Error loading Twitter handle bytes");
    for ch in s.iter() {
      require!(is_valid_twitter_handle_char(*ch), "Invalid character found in Twitter handle");
    }
  }

  fn require_valid_message(&self, message: ManagedBuffer) {
    require!(message.len() <= MESSAGE_MAX_LENGTH, "Message too long");
    let mut str_bytes = [0u8; MESSAGE_MAX_LENGTH];
    let s = &mut str_bytes[..message.len()];
    require!(!message.load_slice(0, s).is_err(), "Error loading message bytes");
    for ch in s.iter() {
      require!(is_valid_message_char(*ch), "Invalid character found in message");
    }
  }

  #[endpoint(mintTierNfts)]
  fn mint_tier_nfts(&self) {
    require!(self.minting_state().get() == MintingState::Active, "Minting not enabled");
    let donor_address = self.blockchain().get_caller();
    let donor_data = self.donors_data(&donor_address).get();
    let mut last_minted_tier_id = donor_data.last_minted_tier_id;
    let tier_thresholds = self.tier_thresholds().get();
    while usize::from(last_minted_tier_id) < tier_thresholds.len() {
      let tier_id_to_mint = last_minted_tier_id + 1;
      let tier_threshold = tier_thresholds.get(usize::from(tier_id_to_mint - 1));
      if donor_data.donation < *tier_threshold {
        break;
      }
      self.collection().nft_add_quantity_and_send(&donor_address, u64::from(tier_id_to_mint), BigUint::from(1u64));
      last_minted_tier_id = tier_id_to_mint;
    }
    self.donors_data(&donor_address).set(DonorData {
      id: donor_data.id,
      pseudo: donor_data.pseudo,
      twitter_handle: donor_data.twitter_handle,
      message: donor_data.message,
      donation_destination_id: donor_data.donation_destination_id,
      donation: donor_data.donation,
      last_minted_tier_id: last_minted_tier_id,
    });
  }

  #[only_owner]
  #[endpoint(sendEgldsToDistributionAddress)]
  fn send_eglds_to_distribution_address(&self) {
    require!(!self.distribution_address().is_empty(), "No specified distribution address");
    let egld_id = TokenIdentifier::egld();
    let egld_amount = self.blockchain().get_sc_balance(&egld_id, 0);
    self.send().direct(&self.distribution_address().get(), &egld_id, 0, &egld_amount, &[]);
  }

  #[endpoint(setMinDonation)]
  #[only_owner]
  fn set_min_donation(&self, min_donation: BigUint) {
    self.min_donation().set(&min_donation);
  }

  #[endpoint(setTierThresholds)]
  #[only_owner]
  fn set_tier_thresholds(&self, tier_thresholds: ManagedVec<BigUint>) {
    self.tier_thresholds().set(&tier_thresholds);
  }

  #[endpoint(setMaxDonationDestinationId)]
  fn set_max_donation_destination_id(&self, max_donation_destination_id: DonationDestinationId) {
    self.max_donation_destination_id().set(&max_donation_destination_id);
  }

  #[endpoint(setDistributionAddress)]
  #[only_owner]
  fn set_distribution_address(&self, distribution_address: ManagedAddress) {
    self.distribution_address().set(&distribution_address);
  }

  #[endpoint(setDonationState)]
  #[only_owner]
  fn set_donation_state(&self, donation_state: DonationState) {
    self.donation_state().set(&donation_state);
  }

  #[endpoint(setMintingState)]
  #[only_owner]
  fn set_minting_state(&self, minting_state: MintingState) {
    self.minting_state().set(&minting_state);
  }

  #[event("donation")]
  fn donation_event(&self, #[indexed] donor_address: ManagedAddress, amount: BigUint);

  #[storage_mapper("min_donation")]
  fn min_donation(&self) -> SingleValueMapper<BigUint>;

  #[storage_mapper("tier_thresholds")]
  fn tier_thresholds(&self) -> SingleValueMapper<ManagedVec<BigUint>>;

  #[storage_mapper("max_donation_destination_id")]
  fn max_donation_destination_id(&self) -> SingleValueMapper<DonationDestinationId>;

  #[storage_mapper("distribution_address")]
  fn distribution_address(&self) -> SingleValueMapper<ManagedAddress>;

  #[storage_mapper("collection")]
  fn collection(&self) -> NonFungibleTokenMapper<Self::Api>;

  #[storage_mapper("donation_state")]
  fn donation_state(&self) -> SingleValueMapper<DonationState>;

  #[storage_mapper("minting_state")]
  fn minting_state(&self) -> SingleValueMapper<MintingState>;

  #[storage_mapper("donors_data")]
  fn donors_data(&self, donor_address: &ManagedAddress) -> SingleValueMapper<DonorData<Self::Api>>;

  #[storage_mapper("total_donation")]
  fn total_donation(&self) -> SingleValueMapper<BigUint>;

  #[storage_mapper("num_donors")]
  fn num_donors(&self) -> SingleValueMapper<u64>;
}

fn is_valid_pseudo_char(ch: u8) -> bool {
  if ch >= b'a' && ch <= b'z' {
    return true;
  }
  if ch >= b'A' && ch <= b'Z' {
    return true;
  }
  if ch >= b'0' && ch <= b'9' {
    return true;
  }
  if ch == b'_' || ch == b' ' {
    return true;
  }
  false
}

fn is_valid_twitter_handle_char(ch: u8) -> bool {
  if ch >= b'a' && ch <= b'z' {
    return true;
  }
  if ch >= b'A' && ch <= b'Z' {
    return true;
  }
  if ch >= b'0' && ch <= b'9' {
    return true;
  }
  if ch == b'_' {
    return true;
  }
  false
}

fn is_valid_message_char(ch: u8) -> bool {
  if ch >= b'a' && ch <= b'z' {
    return true;
  }
  if ch >= b'A' && ch <= b'Z' {
    return true;
  }
  if ch >= b'0' && ch <= b'9' {
    return true;
  }
  if ch == b' ' || ch == b'!' || ch == b'?' || ch == b'\'' || ch == b','
    || ch == b'.' || ch == b':' || ch == b'$' || ch == b'%' || ch == b'-' {
    return true
  }
  false
}

const PSEUDO_MAX_LENGTH: usize = 15;
const TWITTER_HANDLE_MAX_LENGTH: usize = 15;
const MESSAGE_MAX_LENGTH: usize = 100;

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq)]
pub struct DonorData<M: ManagedTypeApi> {
  id: u64,
  pseudo: ManagedBuffer<M>,
  twitter_handle: ManagedBuffer<M>,
  message: ManagedBuffer<M>,
  donation_destination_id: DonationDestinationId,
  donation: BigUint<M>,
  last_minted_tier_id: TierId,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq)]
pub enum DonationState {
  Inactive,
  Active,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq)]
pub enum MintingState {
  Inactive,
  Active,
}

type DonationDestinationId = u8;
type TierId = u8;
