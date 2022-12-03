use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::store::lookup_map::Entry;
use near_sdk::store::LookupMap;
use near_sdk::{env, near_bindgen, AccountId, BorshStorageKey, PanicOnDefault, Promise};

#[derive(BorshSerialize, BorshStorageKey)]
enum StorageKey {
    Donations,
}

#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Debug, PartialEq, Eq)]
#[serde(crate = "near_sdk::serde")]
pub struct Donation {
    /// Donation account id
    pub account_id: AccountId,
    /// Donation sum
    pub sum: U128,
    /// Donation message
    pub msg: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// we can deploy contract to any other account on near and specify main account hear for all transfers
    beneficiary: AccountId,

    /// storage for all donations
    /// <donation_id, Donation>
    donations: LookupMap<U128, Donation>,
    /// sequence growing after insert donation
    donations_sequence: u128,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(beneficiary: AccountId) -> Self {
        Self {
            beneficiary,
            donations: LookupMap::new(StorageKey::Donations),
            donations_sequence: 0,
        }
    }

    #[payable]
    pub fn donate(&mut self, msg: String) {
        match self.donations.entry(U128::from(self.donations_sequence)) {
            Entry::Occupied(_) => env::panic_str("donation id already exist"),
            Entry::Vacant(e) => {
                e.insert(Donation {
                    account_id: env::signer_account_id(),
                    sum: U128::from(env::attached_deposit()),
                    msg,
                });
            }
        }

        self.donations_sequence += 1;

        Promise::new(self.beneficiary.clone()).transfer(env::attached_deposit());
    }

    pub fn get_donation(&self, donation_id: U128) -> Option<&Donation> {
        self.donations.get(&donation_id)
    }

    pub fn donations_sequence(&self) -> U128 {
        U128::from(self.donations_sequence)
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct Web4Request {
    #[serde(rename = "accountId")]
    pub account_id: Option<String>,
    pub path: String,
    #[serde(default)]
    pub params: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub query: std::collections::HashMap<String, Vec<String>>,
    pub preloads: Option<std::collections::HashMap<String, Web4Response>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde", untagged)]
pub enum Web4Response {
    Body {
        #[serde(rename = "contentType")]
        content_type: String,
        body: near_sdk::json_types::Base64VecU8,
    },
    BodyUrl {
        #[serde(rename = "bodyUrl")]
        body_url: String,
    },
    PreloadUrls {
        #[serde(rename = "preloadUrls")]
        preload_urls: Vec<String>,
    },
}

#[near_bindgen]
impl Contract {
    /// Learn more about web4 here: https://web4.near.page
    pub fn web4_get(&self, request: Web4Request) -> Web4Response {
        if request.path == "/stats" {
            return Web4Response::Body {
                content_type: "text/html; charset=UTF-8".to_owned(),
                body: include_str!("stats.html").as_bytes().to_owned().into(),
            };
        }

        if request.path == "/alerts" {
            return Web4Response::Body {
                content_type: "text/html; charset=UTF-8".to_owned(),
                body: include_str!("alerts.html").as_bytes().to_owned().into(),
            };
        }

        Web4Response::Body {
            content_type: "text/html; charset=UTF-8".to_owned(),
            body: include_str!("donation.html").as_bytes().to_owned().into(),
        }
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, Balance, VMContext, ONE_NEAR};

    fn get_context(
        is_view: bool,
        deposit: Balance,
        signer: AccountId,
        predecessor: AccountId,
    ) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(signer)
            .predecessor_account_id(predecessor)
            .is_view(is_view)
            .attached_deposit(deposit)
            .random_seed([1u8; 32])
            .build()
    }

    #[test]
    fn test_donation() {
        let context = get_context(
            false,
            ONE_NEAR,
            "a_near".parse().unwrap(),
            "a_near".parse().unwrap(),
        );

        testing_env!(context);

        let mut contract = Contract::new("b_near".parse().unwrap());

        contract.donate("123".to_string());

        assert_eq!(contract.donations_sequence, 1);
        assert_eq!(
            contract.donations.get(&U128::from(0u128)),
            Some(&Donation {
                account_id: "a_near".parse().unwrap(),
                sum: U128(ONE_NEAR),
                msg: "123".to_string()
            })
        );
    }
}
