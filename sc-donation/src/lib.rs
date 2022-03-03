#![no_std]

elrond_wasm::imports!();
elrond_wasm::derive_imports!();

#[elrond_wasm::contract]
pub trait Donation
{
  #[init]
  fn init(
    &self,
    min_donation: BigUint
  ) {
    self.min_donation().set(&min_donation);
    self.total_donation().set(BigUint::zero());
    self.num_donors().set(0);
    self.state().set(State::Inactive);
    self.tier_thresholds(0).set(BigUint::zero());
    self.max_tier().set(0);
  }

  #[only_owner]
  #[payable("EGLD")]
  #[endpoint(issueNFT)]
  fn issue_nft(
    &self,
    #[payment_amount] issue_cost: BigUint,
    token_name: ManagedBuffer,
    token_ticker: ManagedBuffer,
    tier: u8
  ) {
    require!(self.nft_ids(tier).is_empty(), "NFT already issued for this tier");
    require!(!self.tier_thresholds(tier).is_empty(), "Threshold not yet defined for this tier");
    self.send()
      .esdt_system_sc_proxy()
      .issue_non_fungible(
        issue_cost,
        &token_name,
        &token_ticker,
        NonFungibleTokenProperties {
          can_freeze: true,
          can_wipe: true,
          can_pause: true,
          can_change_owner: true,
          can_upgrade: true,
          can_add_special_roles: true,
        },
      )
      .async_call()
      .with_callback(self.callbacks().issue_nft_callback(&self.blockchain().get_caller(), tier))
      .call_and_exit()
  }

  #[callback]
  fn issue_nft_callback(
    &self,
    caller: &ManagedAddress,
    #[call_result] result: ManagedAsyncCallResult<TokenIdentifier>,
    tier: u8
  ) {
    match result {
      ManagedAsyncCallResult::Ok(token_id) => {
        if self.nft_ids(tier).is_empty() {
          self.nft_ids(tier).set(&token_id);
        }
      }
      ManagedAsyncCallResult::Err(_message) => {
        let (returned_tokens, token_id) = self.call_value().payment_token_pair();
        if token_id.is_egld() && returned_tokens > 0 {
          self.send().direct_egld(caller, &returned_tokens, &[]);
        }
      }
    }
  }

  #[only_owner]
  #[endpoint(setLocalRoles)]
  fn set_local_roles(&self, tier: u8) {
    require!(!self.nft_ids(tier).is_empty(), "NFT not issued");
    let roles = [EsdtLocalRole::NftCreate, EsdtLocalRole::NftBurn];
    self.send()
      .esdt_system_sc_proxy()
      .set_special_roles(
        &self.blockchain().get_sc_address(),
        &self.nft_ids(tier).get(),
        (&roles[..]).into_iter().cloned(),
      )
      .async_call()
      .call_and_exit()
  }

  fn _create_nft(
    &self,
    tier: u8,
    attributes: BigUint,
  ) -> EsdtTokenPayment<Self::Api> {
    let token_id = self.nft_ids(tier).get();
    let amount = BigUint::from(1u64);
    let token_nonce = self.send().esdt_nft_create(
      &token_id,
      &amount,
      &ManagedBuffer::new(),
      &BigUint::zero(),
      &ManagedBuffer::new(),
      &attributes,
      &ManagedVec::new(),
    );
    EsdtTokenPayment::new(token_id, token_nonce, amount)
  }

  #[payable("EGLD")]
  #[endpoint(donate)]
  fn donate(
    &self,
    #[payment_amount] donation: BigUint,
    pseudo: ManagedBuffer,
    twitter_handle: ManagedBuffer,
    message: ManagedBuffer,
  ) {
    require!(self.state().get() == State::Active, "Donations are not enabled");
    let donor_address = self.blockchain().get_caller();
    self._set_donor_data(donor_address.clone(), donation.clone(), pseudo, twitter_handle, message);
    let total_donation = self.total_donation().get();
    self.total_donation().set(total_donation + donation.clone());
    self.donation_event(donor_address, donation);
  }

  fn _set_donor_data(
    &self,
    donor_address: ManagedAddress,
    donation: BigUint,
    pseudo: ManagedBuffer,
    twitter_handle: ManagedBuffer,
    message: ManagedBuffer,
  ) {
    require!(donation >= self.min_donation().get(), "Donation too small");
    self._check_valid_pseudo(pseudo.clone());
    self._check_valid_twitter_handle(twitter_handle.clone());
    self._check_valid_message(message.clone());
    let (id, total_donation, old_tier) =
      if self.donors_data(&donor_address).is_empty() {
        let num_donors = self.num_donors().get() + 1;
        self.num_donors().set(num_donors);
        (num_donors, donation, 0)
      } else {
        let old_donor_data = self.donors_data(&donor_address).get();
        let total_donation = old_donor_data.total_donation + donation;
        (old_donor_data.id, total_donation, old_donor_data.tier)
      };
    let tier = self._compute_new_tier_and_send_nfts(old_tier, total_donation.clone());
    let donor_data = DonorData{
      id: id,
      total_donation: total_donation,
      pseudo: pseudo,
      twitter_handle: twitter_handle,
      message: message,
      tier: tier,
    };
    self.donors_data(&donor_address).set(donor_data);
  }

  fn _compute_new_tier_and_send_nfts(
    &self,
    tier: u8,
    amount: BigUint
  ) -> u8 {
    let mut new_tier = tier;
    while new_tier < self.max_tier().get() {
      if amount >= self.tier_thresholds(new_tier + 1).get() {
        new_tier +=1;
        let nft = self._create_nft(new_tier, BigUint::zero());
        self.send().direct(&self.blockchain().get_caller(), &nft.token_identifier, nft.token_nonce, &nft.amount, &[]);
      }
      else { break; }
    }
    new_tier
  }

  #[only_owner]
  #[endpoint(sendEgldsToDistributionAddress)]
  fn send_eglds_to_distribution_address(
    &self,
  ) {
    require!(!self.distribution_address().is_empty(), "No specified distribution address");
    let egld_id = TokenIdentifier::egld();
    let egld_amount = self.blockchain().get_sc_balance(&egld_id, 0);
    self.send().direct(&self.distribution_address().get(), &egld_id, 0, &egld_amount, &[]);
  }

  fn _check_valid_pseudo(&self, pseudo: ManagedBuffer) {
    require!(pseudo.len() <= PSEUDO_MAX_SIZE, "Pseudo too long");
    let mut str_bytes = [0u8; PSEUDO_MAX_SIZE];
    let s = &mut str_bytes[..pseudo.len()];
    require!(!pseudo.load_slice(0, s).is_err(), "Error loading pseudo bytes");
    for ch in s.iter() {
      require!(self._is_valid_pseudo_char(*ch), "Invalid character found in pseudo");
    }
  }

  fn _check_valid_twitter_handle(&self, twitter_handle: ManagedBuffer) {
    require!(twitter_handle.len() <= TWITTER_HANDLE_MAX_SIZE, "Twitter handle too long");
    let mut str_bytes = [0u8; TWITTER_HANDLE_MAX_SIZE];
    let s = &mut str_bytes[..twitter_handle.len()];
    require!(!twitter_handle.load_slice(0, s).is_err(), "Error loading Twitter handle bytes");
    for ch in s.iter() {
      require!(self._is_valid_twitter_handle_char(*ch), "Invalid character found in Twitter handle");
    }
  }

  fn _check_valid_message(&self, message: ManagedBuffer) {
    require!(message.len() <= MESSAGE_MAX_SIZE, "Message too long");
    let mut str_bytes = [0u8; MESSAGE_MAX_SIZE];
    let s = &mut str_bytes[..message.len()];
    require!(!message.load_slice(0, s).is_err(), "Error loading message bytes");
    for ch in s.iter() {
      require!(self._is_valid_message_char(*ch), "Invalid character found in message");
    }
  }

  #[allow(clippy::manual_range_contains)]
  fn _is_valid_pseudo_char(&self, ch: u8) -> bool {
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

  #[allow(clippy::manual_range_contains)]
  fn _is_valid_twitter_handle_char(&self, ch: u8) -> bool {
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

  #[allow(clippy::manual_range_contains)]
  fn _is_valid_message_char(&self, ch: u8) -> bool {
    if ch >= b'a' && ch <= b'z' {
      return true;
    }
    if ch >= b'A' && ch <= b'Z' {
      return true;
    }
    if ch >= b'0' && ch <= b'9' {
      return true;
    }
    if ch == b' ' || ch == b'(' || ch == b')' || ch == b'!' || ch == b'?'
      || ch == b'"' || ch == b'\'' || ch == b',' || ch == b'.' || ch == b':'
      || ch == b'$' || ch == b'%' || ch == b'-' || ch == b'/' {
      return true
    }
    false
  }

  #[endpoint(setState)]
  #[only_owner]
  fn set_state(
    &self,
    state: State,
  ) {
    self.state().set(&state);
  }

  #[endpoint(setDistributionAddress)]
  #[only_owner]
  fn set_distribution_address(
    &self,
    distribution_address: ManagedAddress,
  ) {
    self.distribution_address().set(&distribution_address);
  }

  #[endpoint(setTierThreshold)]
  #[only_owner]
  fn set_tier_threshold(
    &self,
    tier: u8,
    threshold: BigUint
  ) {
    if tier >= 2 {
      require!(!self.tier_thresholds(tier-1).is_empty(), "Threshold for previous tier undefined")
    }
    if self.tier_thresholds(tier).is_empty() {
      let max_tier = self.max_tier().get();
      self.max_tier().set(max_tier+1);
    }
    self.tier_thresholds(tier).set(&threshold);
  }

  #[event("donation")]
  fn donation_event(
    &self,
    #[indexed] donor_address: ManagedAddress,
    amount: BigUint
  );

  #[storage_mapper("donors_data")]
  fn donors_data(&self, donor_address: &ManagedAddress) -> SingleValueMapper<DonorData<Self::Api>>;

  #[storage_mapper("num_donors")]
  fn num_donors(&self) -> SingleValueMapper<u64>;

  #[storage_mapper("min_donation")]
  fn min_donation(&self) -> SingleValueMapper<BigUint>;

  #[storage_mapper("total_donation")]
  fn total_donation(&self) -> SingleValueMapper<BigUint>;

  #[storage_mapper("distribution_address")]
  fn distribution_address(&self) -> SingleValueMapper<ManagedAddress>;

  #[storage_mapper("state")]
  fn state(&self) -> SingleValueMapper<State>;

  #[storage_mapper("nft_ids")]
  fn nft_ids(&self, tier: u8) -> SingleValueMapper<TokenIdentifier>;

  #[storage_mapper("tier_thresholds")]
  fn tier_thresholds(&self, tier: u8) -> SingleValueMapper<BigUint>;

  #[storage_mapper("max_tier")]
  fn max_tier(&self) -> SingleValueMapper<u8>;
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, PartialEq)]
pub enum State {
  Inactive,
  Active,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, TypeAbi, Clone, PartialEq)]
pub struct DonorData<M: ManagedTypeApi> {
  pub id: u64,
  pub total_donation: BigUint<M>,
  pub pseudo: ManagedBuffer<M>,
  pub twitter_handle: ManagedBuffer<M>,
  pub message: ManagedBuffer<M>,
  pub tier: u8
}

pub const PSEUDO_MAX_SIZE: usize = 15;
pub const TWITTER_HANDLE_MAX_SIZE: usize = 15;
pub const MESSAGE_MAX_SIZE: usize = 180;
