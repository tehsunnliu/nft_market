use openbrush::{traits::{String, AccountId, Balance}, contracts::psp34::PSP34Error};

// #[openbrush::wrapper]
// pub type MintRef = dyn Mint;

#[openbrush::trait_definition]
pub trait Mint {
    /// Mint token
    #[ink(message, payable)]
    fn mint(&mut self, fid: String) -> Result<(), PSP34Error>;

    /// Mint token to
    #[ink(message, payable)]
    fn mint_to(&mut self, to: AccountId, fid: String) -> Result<(), PSP34Error>;

    /// Set new value for the baseUri
    #[ink(message)]
    fn set_base_uri(&mut self, uri: String) -> Result<(), PSP34Error>;

    /// Withdraws funds to contract owner
    fn withdraw(&mut self) -> Result<(), PSP34Error>;

    /// Get URI from token ID
    #[ink(message)]
    fn token_uri(&self, fid: String) -> Result<String, PSP34Error>;

    /// Get token price
    #[ink(message)]
    fn price(&self) -> Balance;

    /// Get max supply of tokens
    #[ink(message)]
    fn max_supply(&self) -> u64;

    /// Set max supply of tokens
    #[ink(message)]
    fn set_max_supply(&self, value: u64) -> Result<(), PSP34Error>;

    /// Lists NFT for Sale
    #[ink(message)] 
    fn list(&mut self, fid: String, price: Balance) -> Result<(), PSP34Error>;

    /// Delist NFT from Sale
    #[ink(message)] 
    fn delist(&mut self, fid: String) -> Result<(), PSP34Error>;

    /// Purchase NFT that is listed for Sale 
    #[ink(message, payable)]
    fn purchase(&mut self, fid: String) -> Result<(), PSP34Error>;
}
