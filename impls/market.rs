use ink::codegen::EmitEvent;
use openbrush::{
    contracts::{
        ownable::{self, only_owner},
        psp34::{
            self,
            extensions::metadata::{self, PSP34MetadataImpl},
            Id, PSP34Error, PSP34Impl,
        },
        reentrancy_guard,
        reentrancy_guard::non_reentrant,
    },
    modifiers,
    traits::{AccountId, Balance, Storage, String},
};

use super::types::{NftData, NftError};

#[openbrush::trait_definition]
pub trait MintImpl:
    Storage<NftData>
    + Storage<psp34::Data>
    + Storage<reentrancy_guard::Data>
    + Storage<ownable::Data>
    + Storage<metadata::Data>
    + PSP34Impl
    + PSP34MetadataImpl
    + psp34::extensions::metadata::Internal
    + Internal
{
    /// Mint token to
    #[ink(message, payable)]
    #[modifiers(non_reentrant)]
    fn mint(&mut self, fid: String) -> Result<(), PSP34Error> {
        self.check_fid(fid.clone())?;
        let caller = Self::env().caller();
        self._mint_to(caller, Id::Bytes(fid.into_bytes()))?;
        Ok(())
    }

    /// Mint token to
    #[ink(message, payable)]
    #[modifiers(non_reentrant)]
    fn mint_to(&mut self, to: AccountId, fid: String) -> Result<(), PSP34Error> {
        self.check_fid(fid.clone())?;
        self._mint_to(to, Id::Bytes(fid.into_bytes()))?;
        Ok(())
    }

    /// Set new value for the baseUri
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error> {
        let id = PSP34Impl::collection_id(self);
        metadata::Internal::_set_attribute(self, id, String::from("baseUri"), uri);
        Ok(())
    }

    /// Get URI from token ID
    #[ink(message)]
    fn token_uri(&self, fid: String) -> Result<String, PSP34Error> {
        self.token_exists(Id::Bytes(fid.clone().into_bytes()))?;
        let base_uri = PSP34MetadataImpl::get_attribute(
            self,
            PSP34Impl::collection_id(self),
            String::from("baseUri"),
        );
        let token_uri = base_uri.unwrap() + &fid;
        Ok(token_uri)
    }

    /// Get token price
    #[ink(message)]
    fn price(&self) -> Balance {
        self.data::<NftData>().price_per_mint
    }

    /// Get max supply of tokens
    #[ink(message)]
    fn max_supply(&self) -> u64 {
        self.data::<NftData>().max_supply
    }

    /// Set max supply of tokens
    #[ink(message)]
    #[modifiers(only_owner)]
    fn set_max_supply(&mut self, value: u64) -> Result<(), PSP34Error> {
        self.data::<NftData>().max_supply = value;
        Ok(())
    }

    /// Lists NFT for Sale
    #[ink(message)]
    fn list(&mut self, fid: String, price: Balance) -> Result<(), PSP34Error> {
        let id = Id::Bytes(fid.into_bytes());
        self.check_owner(id.clone())?;
        self.data::<NftData>().sale_list.insert(&id, &price);
        Ok(())
    }

    /// Delist NFT from Sale
    #[ink(message)]
    fn delist(&mut self, fid: String) -> Result<(), PSP34Error> {
        let id = Id::Bytes(fid.into_bytes());
        self.check_owner(id.clone())?;
        if self.data::<NftData>().sale_list.get(&id).is_none() {
            return Err(PSP34Error::Custom(NftError::NotForSale.as_str()));
        }
        self.data::<NftData>().sale_list.remove(&id);
        Ok(())
    }

    /// Purchase NFT that is listed for Sale
    #[ink(message, payable)]
    fn purchase(&mut self, fid: String) -> Result<(), PSP34Error> {
        let id = Id::Bytes(fid.into_bytes());
        let owner = self._check_token_exists(&id.clone())?;
        let caller = Self::env().caller();
        if owner == caller {
            return Err(PSP34Error::Custom(NftError::OwnToken.as_str()));
        };

        let price = self
            .data::<NftData>()
            .sale_list
            .get(&id)
            .ok_or(PSP34Error::Custom(NftError::NotForSale.as_str()))?;
        if price != Self::env().transferred_value() {
            return Err(PSP34Error::Custom(NftError::PriceNotMatch.as_str()));
        }

        // Transfer native tokes
        if Self::env().transfer(owner, price).is_err() {
            return Err(PSP34Error::Custom(
                NftError::TransferNativeTokenFailed.as_str(),
            ));
        }

        self.data::<NftData>().sale_list.remove(&id);

        // Transfer NFT Token
        self._before_token_transfer(Some(&owner), Some(&caller), &id)?;
        self._remove_token(&owner, &id)?;
        self._do_safe_transfer_check(&owner, &owner, &caller, &id, &vec![])?;
        self._add_token(&caller, &id)?;
        self._after_token_transfer(Some(&owner), Some(&caller), &id)?;
        self._emit_transfer_event(Some(owner), Some(caller), id.clone());

        Self::env().emit_event(Trade {
            seller: owner,
            buyer: caller,
            id,
            price,
        });

        Ok(())
    }
}

pub trait Internal: Storage<NftData> + psp34::Internal {
    /// Check if the caller is owner of the token
    fn check_owner(&self, id: Id) -> Result<(), PSP34Error> {
        let owner = self._check_token_exists(&id.clone())?;
        let caller = Self::env().caller();
        if owner != caller {
            return Err(PSP34Error::Custom(NftError::NotTokenOwner.as_str()));
        }
        Ok(())
    }

    /// Check if the transferred mint value is as expected
    fn check_value(&self, transferred_value: u128) -> Result<(), PSP34Error> {
        if transferred_value == self.data::<NftData>().price_per_mint {
            return Ok(());
        }
        Err(PSP34Error::Custom(NftError::BadMintValue.as_str()))
    }

    fn check_fid(&self, fid: String) -> Result<(), PSP34Error> {
        // TODO: Check if fid exists in CESS Chain.
        Ok(())
    }

    fn token_exists(&self, id: Id) -> Result<(), PSP34Error> {
        self._owner_of(&id).ok_or(PSP34Error::TokenNotExists)?;
        Ok(())
    }
}