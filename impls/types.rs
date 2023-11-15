use openbrush::{traits::{Balance, String}, storage::Mapping, contracts::psp34::Id};


#[derive(Default, Debug)]
#[openbrush::storage_item]
pub struct NftData {
    pub collection_id: u32,
    pub max_supply: u64,
    pub price_per_mint: Balance, 
    pub sale_list: Mapping<Id, Balance>,
}

#[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub enum NftError {
    BadMintValue,
    CannotMintZeroTokens,
    CollectionIsFull,
    TooManyTokensToMint,
    WithdrawalFailed,
    NotTokenOwner,
    NotForSale,
    OwnToken,
    PriceNotMatch,
    TransferNativeTokenFailed,
}

impl NftError {
    pub fn as_str(&self) -> String {
        match self {
            NftError::BadMintValue => String::from("BadMintValue"),
            NftError::CannotMintZeroTokens => String::from("CannotMintZeroTokens"),
            NftError::CollectionIsFull => String::from("CollectionIsFull"),
            NftError::TooManyTokensToMint => String::from("TooManyTokensToMint"),
            NftError::WithdrawalFailed => String::from("WithdrawalFailed"),
            NftError::NotTokenOwner => String::from("NotTokenOwner"),
            NftError::NotForSale => String::from("NotForSale"),
            NftError::OwnToken => String::from("OwnToken"),
            NftError::PriceNotMatch => String::from("PriceNotMatch"),
            NftError::TransferNativeTokenFailed => String::from("TransferNativeTokenFailed"),
        }
    }
}