#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract]
mod erc20 {

    use ink_storage::collections::HashMap as StorageHashMap;


    /// Defines the storage of your contract.
    /// Add new fields to the below struct in order
    /// to add new static storage fields to your contract.
    #[ink(storage)]
    pub struct Erc20 {
        /// Stores a single `bool` value on the storage.
        total_supply: Balance,
        owner:AccountId,
        balances:StorageHashMap<AccountId,Balance>,
        allowance:StorageHashMap<(AccountId,AccountId),Balance>
    }
    #[ink(event)]
    pub struct Transfer{
        #[ink(topic)]
        from:AccountId,
        #[ink(topic)]
        to:AccountId,
        value:Balance
    }
    #[ink(event)]
    pub struct  Allowance{
        #[ink(topic)]
        from:AccountId,
        #[ink(topic)]
        to:AccountId,
        value:Balance
    }
    #[ink(event)]
    pub struct Burn{
        #[ink(topic)]
        from:AccountId,
        #[ink(topic)]
        value:Balance
    }
    #[ink(event)]
    pub struct Issue{
        #[ink(topic)]
        value:Balance
    }


    #[derive(Debug,Eq,PartialEq,scale::Encode)]
    #[cfg_attr(feature = "std",derive(scale_info::TypeInfo))]
    pub enum Error{
        InSufficientBalance,
        NotContractOwner

    }


    pub type Result<T> = core::result::Result<T,Error>;

    impl Erc20 {
        /// Constructor that initializes the `bool` value to the given `init_value`.
        #[ink(constructor)]
        pub fn new(total_supply:Balance ) -> Self {
            let caller = Self::env().caller();
            let mut balances = StorageHashMap::new();
            balances.insert(caller,total_supply);
            let instance = Self{
                total_supply : total_supply,
                balances: balances,
                allowance:StorageHashMap::new(),
                owner:caller,
            };

            instance
        }
       #[ink(message)]
       pub fn total_supply(&self)->Balance{
           self.total_supply
       }

        #[ink(message)]
        pub fn balance_of(&self,owner:AccountId)->Balance{
            *self.balances.get(&owner).unwrap_or(&0)
        }
        #[ink(message)]
        pub fn allowance(&self,owner:AccountId,spender:AccountId)->Balance{
            *self.allowance.get(&(owner,spender)).unwrap_or(&0)
        }

        #[ink(message)]
        pub fn allowance_to(&mut self,to:AccountId,value:Balance)->Result<()>{
            let who = Self::env().caller();

            let from_balance = self.balance_of(who);

            let allowance = self.allowance(who,to);

            if from_balance < (allowance + value) {
                return Err(Error::InSufficientBalance);
            }
            self.allowance.insert((who,to),allowance+value);
            self.env().emit_event(
                Allowance{
                    from:who,
                    to,
                    value
                }
            );
            Ok(())
        }

        #[ink(message)]
        pub fn burn(&mut self,value:Balance)->Result<()>{
            let who = Self::env().caller();
            let from_balance = self.balance_of(who);
            if from_balance<value {
                return Err(Error::InSufficientBalance);
            }
            self.balances.insert(who,from_balance-value);
            self.total_supply = self.total_supply -value;

            self.env().emit_event( Burn{
                from:who,
                value:value
            } );
            Ok(())

        }
        #[ink(message)]
        pub fn issue (&mut self,value:Balance)->Result<()>{
            let who = Self::env().caller();
            if who!=self.owner{
                return Err(Error::NotContractOwner);
            }
            self.total_supply = self.total_supply + value;
            let balance = self.balance_of(who);
            self.balances.insert(who,balance+value);
            self.env().emit_event(Issue{
                value
            });
            Ok(())
        }

        #[ink(message)]
        pub fn transfer(&mut self,to:AccountId,value:Balance)->Result<()>{
            let who = Self::env().caller();
            self.transfer_help(who,to,value)
        }

        fn transfer_help(&mut self, from:AccountId,to:AccountId,value:Balance)->Result<()>{
            let from_balance = self.balance_of(from);
            if from_balance < value{
                return Err(Error::InSufficientBalance);
            }
            self.balances.insert(from,from_balance-value);
            let to_balance = self.balance_of(to);
            self.balances.insert(to,to_balance+value);
            self.env().emit_event(
                Transfer{
                    from,to,value
                }
            );
            Ok(())

        }

        // Simply returns the current value of our `bool`.
        // #[ink(message)]
        // pub fn get(&self) -> bool {
        //     self.value
        // }
    }

    /// Unit tests in Rust are normally defined within such a `#[cfg(test)]`
    /// module and test functions are marked with a `#[test]` attribute.
    /// The below code is technically just normal Rust code.
    #[cfg(test)]
    mod tests {
        /// Imports all the definitions from the outer scope so we can use them here.
        use super::*;

        /// We test if the default constructor does its job.
        #[test]
        fn create_contract_works() {
            let erc20 = Erc20::new(100);

            assert_eq!(erc20.total_supply, 100);
        }


    }
}
