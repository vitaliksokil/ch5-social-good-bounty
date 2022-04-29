use std::collections::HashMap;
use near_sdk::{PromiseOrValue, Promise, near_bindgen, PanicOnDefault, BorshStorageKey, AccountId, borsh::{self, BorshDeserialize, BorshSerialize}, serde::{Deserialize, Serialize}, log, env};
use near_contract_standards::non_fungible_token::NonFungibleToken;
use near_contract_standards::non_fungible_token::{metadata::NFTContractMetadata, Token, TokenId};
use near_contract_standards::non_fungible_token::metadata::{NFT_METADATA_SPEC, NonFungibleTokenMetadataProvider, TokenMetadata};
use near_sdk::collections::{LazyOption, UnorderedMap};

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub enum DonationType {
    Medicine,
    Children,
    Disability,
    Environment,
    Animal,
    Education,
}

#[derive(BorshDeserialize, BorshSerialize, Deserialize, Serialize, Debug, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct Meeting {
    owner_id: AccountId,
    owner_name: String,
    title: String,
    description: String,
    // in yoctoNear 5_000_000_000_000_000_000_000_000 = 5 Ⓝ
    ticket_price: u128,
    total_tickets: u16,
    bought_tickets: u16,
    place:String,
    donation_type:DonationType,
    nft_media:String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    token: NonFungibleToken,
    metadata: LazyOption<NFTContractMetadata>,
    total_meetings: u8,
    pub meetings: UnorderedMap<u8, Meeting>
}

#[derive(BorshSerialize, BorshStorageKey)]
pub enum StorageKey {
    NonFungibleToken,
    Metadata,
    TokenMetadata,
    Enumeration,
    Approval
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: AccountId) -> Self {
        Self {
            token: NonFungibleToken::new(
                StorageKey::NonFungibleToken,
                owner_id,
                Some(StorageKey::TokenMetadata),
                Some(StorageKey::Enumeration),
                Some(StorageKey::Approval)
            ),
            metadata: LazyOption::new(
                StorageKey::Metadata,
                Some(&NFTContractMetadata {
                    spec: NFT_METADATA_SPEC.to_string(),
                    name: "MeetingBuy".to_string(),
                    symbol: "Example".to_string(),
                    icon: Some("ANY_SVG".to_string()),
                    base_uri: None,
                    reference: None,
                    reference_hash: None,
                })
            ),
            total_meetings: 0,
            meetings: UnorderedMap::new(b"d".to_vec())
        }
    }


    pub fn get_all_meetings(&self) -> HashMap<u8,Meeting>{
        self.meetings.iter().collect()
    }

    pub fn get_meeting_by_id(&self, id: u8) -> Meeting {
        self.meetings.get(&id).expect("Meeting doesn't exist")
    }

    pub fn get_meetings_by_owner(&self, owner_account_id: AccountId) -> HashMap<u8, Meeting>{
        let all_meetings: HashMap<u8, Meeting> = self.get_all_meetings();
        let mut result: HashMap<u8, Meeting> = HashMap::new();
        for (key, val) in all_meetings.iter() {
            if val.owner_id == owner_account_id {
                result.insert(*key, val.clone());
            }
        }
        result
    }

    pub fn add_new_meeting(
        &mut self,
        owner_name: String,
        title: String,
        description: String,
        ticket_price: String,
        total_tickets:u16,
        place: String,
        donation_type:DonationType,
        nft_media:String,
    ) -> Meeting{
        let ticket_price_u128 = ticket_price.parse::<u128>().unwrap();

        assert!(title != "", "Abort. Title is empty");
        assert!(title.len() <= 1000, "Abort. Title is longer then 1000 characters");
        assert!(description.len() <= 2000, "Abort. Description is longer then 2000 characters");
        assert!(place != "", "Abort. Place is empty");
        assert!(place.len() <= 1000, "Abort. Place is longer then 1000 characters");

        self.total_meetings += 1;
        let id: u8 = self.total_meetings;

        let owner_account_id: AccountId = env::predecessor_account_id();


        self.meetings.insert(
            &id,
            &Meeting {
                owner_id: owner_account_id,
                owner_name,
                title,
                description,
                ticket_price: ticket_price_u128,
                total_tickets,
                bought_tickets: 0,
                place,
                donation_type,
                nft_media,
            },
        );
        self.get_meeting_by_id(id)
    }

    #[payable]
    pub fn buy_ticket(&mut self,meeting_id: u8){
        let deposit: u128 = near_sdk::env::attached_deposit();
        let mut meeting: Meeting = self.get_meeting_by_id(meeting_id);
        log!("dep {} ", deposit);
        log!("ticket price {} ", meeting.ticket_price);
        assert_eq!(deposit, meeting.ticket_price, "Incorrect attached deposit");
        assert_ne!(meeting.bought_tickets, meeting.total_tickets, "All tickets sold!");


        meeting.bought_tickets += 1;
        self.meetings.remove(&meeting_id);
        self.meetings.insert(&meeting_id,&meeting);


        let receiver_id: AccountId = env::predecessor_account_id();

        let seed = near_sdk::env::random_seed();
        self.nft_mint(
            receiver_id.to_string() + &meeting_id.to_string() + &'-'.to_string() + &seed[0].to_string() + &seed[1].to_string() + &seed[2].to_string(),
                      receiver_id,
            TokenMetadata{
                title: Option::from(meeting.title),
                description: Option::from(meeting.description),
                media: Option::from(meeting.nft_media),
                media_hash: None,
                copies: None,
                issued_at: None,
                expires_at: None,
                starts_at: None,
                updated_at: None,
                extra: None,
                reference: None,
                reference_hash: None
            });

        Promise::new(meeting.owner_id).transfer(deposit);

    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        receiver_id: AccountId,
        token_metadata: TokenMetadata,
    ) -> Token {
        self.token.internal_mint(token_id, receiver_id, Some(token_metadata))
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, token);
near_contract_standards::impl_non_fungible_token_approval!(Contract, token);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, token);

#[near_bindgen]
impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        self.metadata.get().unwrap()
    }
}


#[cfg(test)]
mod tests {

}
